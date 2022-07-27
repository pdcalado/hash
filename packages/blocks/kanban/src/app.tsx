import {
  BlockComponent,
  Entity,
  useGraphBlockService,
} from "@blockprotocol/graph";
import { useCallback, useRef } from "react";

import styles from "./base.module.scss";
import { Column } from "./column";
import { ColumnProperties, GetEntityFn } from "./types";
import { getLinkedEntities } from "./shared";

type BlockEntityProperties = {
  title: string;
};

export const App: BlockComponent<BlockEntityProperties> = ({
  graph: {
    blockEntity: { entityId, properties = { title: "", columns: [] } },
    blockGraph,
  },
}) => {
  const blockRootRef = useRef<HTMLDivElement>(null);
  const { graphService } = useGraphBlockService(blockRootRef);

  const { title } = properties;

  const getEntities: GetEntityFn = useCallback(
    <T extends Entity>(sourceEntityId: string, path: string) =>
      getLinkedEntities<T>({ sourceEntityId, blockGraph, path }),
    [blockGraph],
  );

  const columns = getEntities<Entity<ColumnProperties>>(entityId, "columns");

  return (
    <div className={styles.block} ref={blockRootRef}>
      <div>
        <h1>{title}</h1>
      </div>
      <div>
        {columns.map((columnEntity) => (
          <Column
            key={columnEntity.properties.title}
            entity={columnEntity}
            getEntities={getEntities}
          />
        ))}
      </div>
    </div>
  );
};
