import { BlockComponent, useGraphBlockService } from "@blockprotocol/graph";
import { useCallback, useRef } from "react";

import styles from "./base.module.scss";
import { Column } from "./column";
import { Column as ColumnType } from "./types";

type BlockEntityProperties = {
  title: string;
  columns: ColumnType[];
};

export const App: BlockComponent<BlockEntityProperties> = ({
  graph: {
    blockEntity: { entityId, properties = { title: "", columns: [] } },
  },
}) => {
  const blockRootRef = useRef<HTMLDivElement>(null);
  const { graphService } = useGraphBlockService(blockRootRef);

  const updateSelf = useCallback(
    (newProperties: Partial<BlockEntityProperties>) =>
      graphService?.updateEntity({
        data: { properties: newProperties, entityId },
      }),
    [entityId, graphService],
  );

  const { title, columns } = properties;

  return (
    <div className={styles.block} ref={blockRootRef}>
      <div>
        <h1>{title}</h1>
      </div>
      <div>
        {columns.map((column) => (
          <Column key={column.title} {...column} />
        ))}
      </div>
    </div>
  );
};
