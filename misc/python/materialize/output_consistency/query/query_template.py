# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.
from collections.abc import Callable

from materialize.output_consistency.execution.evaluation_strategy import (
    EvaluationStrategy,
)
from materialize.output_consistency.execution.query_output_mode import (
    QueryOutputMode,
    query_output_mode_to_sql,
)
from materialize.output_consistency.execution.sql_dialect_adjuster import (
    SqlDialectAdjuster,
)
from materialize.output_consistency.execution.value_storage_layout import (
    ROW_INDEX_COL_NAME,
    ValueStorageLayout,
)
from materialize.output_consistency.expression.expression import Expression
from materialize.output_consistency.expression.expression_characteristics import (
    ExpressionCharacteristics,
)
from materialize.output_consistency.query.query_format import QueryOutputFormat
from materialize.output_consistency.selection.selection import (
    DataRowSelection,
    QueryColumnByIndexSelection,
)


class QueryTemplate:
    """Query template as base for creating SQL for different evaluation strategies"""

    def __init__(
        self,
        expect_error: bool,
        select_expressions: list[Expression],
        where_expression: Expression | None,
        storage_layout: ValueStorageLayout,
        contains_aggregations: bool,
        row_selection: DataRowSelection,
        offset: int | None = None,
        limit: int | None = None,
        custom_db_object_name: str | None = None,
        custom_order_expressions: list[Expression] | None = None,
    ) -> None:
        assert storage_layout != ValueStorageLayout.ANY
        self.expect_error = expect_error
        self.select_expressions: list[Expression] = select_expressions
        self.where_expression = where_expression
        self.storage_layout = storage_layout
        self.contains_aggregations = contains_aggregations
        self.row_selection = row_selection
        self.offset = offset
        self.limit = limit
        self.custom_db_object_name = custom_db_object_name
        self.custom_order_expressions = custom_order_expressions
        self.disable_error_message_validation = not self.__can_compare_error_messages()

    def to_sql(
        self,
        strategy: EvaluationStrategy,
        output_format: QueryOutputFormat,
        query_column_selection: QueryColumnByIndexSelection,
        query_output_mode: QueryOutputMode,
        override_db_object_name: str | None = None,
    ) -> str:
        db_object_name = self.get_db_object_name(strategy, override_db_object_name)
        space_separator = self._get_space_separator(output_format)

        column_sql = self._create_column_sql(
            query_column_selection, space_separator, strategy.sql_adjuster
        )
        where_clause = self._create_where_clause(strategy.sql_adjuster)
        order_by_clause = self._create_order_by_clause(strategy.sql_adjuster)
        limit_clause = self._create_limit_clause()
        offset_clause = self._create_offset_clause()

        explain_mode = query_output_mode_to_sql(query_output_mode)

        sql = f"""
{explain_mode} SELECT{space_separator}{column_sql}
FROM{space_separator}{db_object_name}
{where_clause}
{order_by_clause}
{limit_clause}
{offset_clause};
""".strip()

        return self._post_format_sql(sql, output_format)

    def get_db_object_name(
        self, strategy: EvaluationStrategy, override_db_object_name: str | None = None
    ) -> str:
        return (
            override_db_object_name
            or self.custom_db_object_name
            or strategy.get_db_object_name(self.storage_layout)
        )

    def _get_space_separator(self, output_format: QueryOutputFormat) -> str:
        return "\n  " if output_format == QueryOutputFormat.MULTI_LINE else " "

    def _create_column_sql(
        self,
        query_column_selection: QueryColumnByIndexSelection,
        space_separator: str,
        sql_adjuster: SqlDialectAdjuster,
    ) -> str:
        expressions_as_sql = []
        for index, expression in enumerate(self.select_expressions):
            if query_column_selection.is_included(index):
                expressions_as_sql.append(expression.to_sql(sql_adjuster, True))

        return f",{space_separator}".join(expressions_as_sql)

    def _create_where_clause(self, sql_adjuster: SqlDialectAdjuster) -> str:
        where_conditions = []

        row_filter_clause = self._create_row_filter_clause()
        if row_filter_clause:
            where_conditions.append(row_filter_clause)

        if self.where_expression:
            where_conditions.append(self.where_expression.to_sql(sql_adjuster, True))

        if len(where_conditions) == 0:
            return ""

        # It is important that the condition parts are in parentheses so that they are connected with AND.
        # Otherwise, a generated condition containing OR at the top level may lift the row filter clause.
        all_conditions_sql = " AND ".join(
            [f"({condition})" for condition in where_conditions]
        )
        return f"WHERE {all_conditions_sql}"

    def _create_row_filter_clause(self) -> str | None:
        """Create s SQL clause to only include rows of certain indices"""
        if self.row_selection.keys is None:
            return None

        if len(self.row_selection.keys) == 0:
            row_index_string = "-1"
        else:
            row_index_string = ", ".join(
                str(index) for index in self.row_selection.keys
            )
        return f"{ROW_INDEX_COL_NAME} IN ({row_index_string})"

    def _create_order_by_clause(self, sql_adjuster: SqlDialectAdjuster) -> str:
        if self.custom_order_expressions is not None:
            order_by_specs_str = ", ".join(
                [
                    f"{expr.to_sql(sql_adjuster, True)} ASC"
                    for expr in self.custom_order_expressions
                ]
            )
            return f"ORDER BY {order_by_specs_str}"

        if (
            self.storage_layout == ValueStorageLayout.VERTICAL
            and not self.contains_aggregations
        ):
            return f"ORDER BY {ROW_INDEX_COL_NAME} ASC"

        return ""

    def _create_offset_clause(self) -> str:
        if self.offset is not None:
            return f"OFFSET {self.offset}"

        return ""

    def _create_limit_clause(self) -> str:
        if self.limit is not None:
            return f"LIMIT {self.limit}"

        return ""

    def _post_format_sql(self, sql: str, output_format: QueryOutputFormat) -> str:
        # apply this replacement twice
        sql = sql.replace("\n\n", "\n").replace("\n\n", "\n")
        sql = sql.replace("\n;", ";")

        if output_format == QueryOutputFormat.SINGLE_LINE:
            sql = sql.replace("\n", " ")

        return sql

    def column_count(self) -> int:
        return len(self.select_expressions)

    def __can_compare_error_messages(self) -> bool:
        if self.storage_layout == ValueStorageLayout.HORIZONTAL:
            return True

        for expression in self.select_expressions:
            if expression.contains_leaf_not_directly_consumed_by_aggregation():
                # The query operates on multiple rows and contains at least one non-aggregate function directly
                # operating on the value. Since the row processing order is not fixed, different evaluation
                # strategies may yield different error messages (depending on the first invalid value they
                # encounter). Therefore, error messages shall not be compared in case of a query failure.
                return False

        return True

    def matches_any_select_expression(
        self, predicate: Callable[[Expression], bool], check_recursively: bool
    ) -> bool:
        for expression in self.select_expressions:
            if expression.matches(predicate, check_recursively):
                return True

        return False

    def matches_any_expression(
        self, predicate: Callable[[Expression], bool], check_recursively: bool
    ) -> bool:
        return self.matches_any_select_expression(predicate, check_recursively) or (
            self.where_expression is not None
            and self.where_expression.matches(predicate, check_recursively)
        )

    def matches_specific_select_or_filter_expression(
        self,
        select_column_index: int,
        predicate: Callable[[Expression], bool],
        check_recursively: bool,
    ) -> bool:
        assert 0 <= select_column_index <= self.column_count()
        return self.select_expressions[select_column_index].matches(
            predicate, check_recursively
        ) or (
            self.where_expression is not None
            and self.where_expression.matches(predicate, check_recursively)
        )

    def get_involved_characteristics(
        self,
        query_column_selection: QueryColumnByIndexSelection,
    ) -> set[ExpressionCharacteristics]:
        all_involved_characteristics: set[ExpressionCharacteristics] = set()

        for index, expression in enumerate(self.select_expressions):
            if not query_column_selection.is_included(index):
                continue

            characteristics = expression.recursively_collect_involved_characteristics(
                self.row_selection
            )
            all_involved_characteristics = all_involved_characteristics.union(
                characteristics
            )

        return all_involved_characteristics
