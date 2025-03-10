import { AggregateOperationInput } from "@blockprotocol/graph";
import { FunctionComponent, ReactNode } from "react";
import { ColumnInstance } from "react-table";
import { tw } from "twind";

import { FilterDetail } from "./filter-detail";
import { Menu } from "./menu";
// import { SearchIcon } from "./icons";
import { SortDetail } from "./sort-detail";
import { ToggleColumnsDetail } from "./toggle-columns-detail";

export type AggregateArgs = {
  operation: "filter" | "sort" | "changePageSize";
} & AggregateOperationInput;

type HeaderProps = {
  columns: ColumnInstance<{}>[];
  onAggregate: (args: AggregateArgs) => void;
  toggleHideColumn: (columnId: string) => void;
  aggregateOptions: Pick<AggregateOperationInput, "multiFilter" | "multiSort">;
  entityTypeDropdown: ReactNode;
  entityTypeId: string;
};

export const Header: FunctionComponent<HeaderProps> = ({
  onAggregate,
  columns,
  toggleHideColumn,
  aggregateOptions,
  entityTypeDropdown,
  entityTypeId,
}) => {
  return (
    <div className={tw`pb-3 relative z-0`}>
      <div className={tw`flex items-center`}>
        <div className={tw`mr-3 flex-grow-1`}>{entityTypeDropdown}</div>
        <div className={tw`mr-3`}>
          <Menu label="Filter">
            <FilterDetail
              columns={columns}
              onFilter={(filters) => {
                if (!entityTypeId) {
                  return;
                }

                onAggregate({
                  operation: "filter",
                  multiFilter: filters,
                  entityTypeId,
                });
              }}
              multiFilter={aggregateOptions.multiFilter}
            />
          </Menu>
        </div>
        <div className={tw`mr-3`}>
          <Menu label="Sort">
            <SortDetail
              columns={columns}
              onSort={(sortFields) => {
                if (!entityTypeId) {
                  return;
                }

                onAggregate({
                  operation: "sort",
                  multiSort: sortFields,
                  entityTypeId,
                });
              }}
              multiSort={aggregateOptions.multiSort}
            />
          </Menu>
        </div>
        <div className={tw`mr-3`}>
          <Menu label="Toggle Columns">
            <ToggleColumnsDetail
              columns={columns}
              toggleHideColumn={toggleHideColumn}
            />
          </Menu>
        </div>
        {/* The search input is hidden for now since it doesn't work at the moment
          and it's functions can be performed with the filter button.
          @todo: uncomment when it works
        */}
        {/* <div className={tw`relative w-36`}>
          <input
            type="text"
            className={tw`block w-full max-w-full border-1 rounded pl-6 px-2 py-1 text-sm focus:outline-none hover:border-blue-300 focus:border-blue-500`}
            placeholder="Search"
          />
          <SearchIcon
            className={tw`text-gray-500 h-3.5 w-3.5 absolute left-1 top-0 translate-y-1/2`}
          />
        </div> */}
      </div>
    </div>
  );
};
