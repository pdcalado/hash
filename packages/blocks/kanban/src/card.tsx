import { ReactElement } from "react";

import { Card as CardType } from "./types";

type CardProps = CardType;

export const Card = ({
  assignee,
  description,
  stage,
  title,
}: CardProps): ReactElement | null => {
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
        {assignee.name}
      </div>
    </div>
  );
};
