import { Entity } from "@blockprotocol/graph";
import { ReactElement } from "react";

import { CardProperties, GetEntityFn, OwnerProperties } from "./types";

export const Card = ({
  entity: { entityId, properties },
  getEntities,
}: {
  entity: Entity<CardProperties>;
  getEntities: GetEntityFn;
}): ReactElement | null => {
  const { title, description, stage } = properties;
  const assignee = getEntities<Entity<OwnerProperties>>(entityId, "owner")?.[0];

  return (
    <div style={{ border: "1px solid black", padding: 20 }}>
      <h4>{title}</h4>
      <p>{description}</p>
      <div>
        <strong>Stage: </strong>
        {stage}
      </div>
      <div>
        <strong>Assignee: </strong>
        <span>{assignee?.properties.name ?? "unassigned"}</span>
      </div>
    </div>
  );
};
