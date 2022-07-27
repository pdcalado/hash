import { Entity } from "@blockprotocol/graph";
import { ReactElement } from "react";

import { Card } from "./card";
import { CardProperties, ColumnProperties, GetEntityFn } from "./types";

export const Column = ({
  entity: { entityId, properties },
  getEntities,
}: {
  entity: Entity<ColumnProperties>;
  getEntities: GetEntityFn;
}): ReactElement | null => {
  const { rules, title } = properties;

  const cards = getEntities<Entity<CardProperties>>(entityId, "cards") ?? [];

  return (
    <div style={{ border: "1px solid red", padding: 15 }}>
      <h3>{title}</h3>
      {cards.map((cardEntity) => (
        <Card
          key={cardEntity.properties.title}
          entity={cardEntity}
          getEntities={getEntities}
        />
      ))}
    </div>
  );
};
