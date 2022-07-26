import { ReactElement } from "react";

import { Column as ColumnType } from "./types";
import { Card } from "./card";

type ColumnProps = ColumnType;

export const Column = ({
  cards,
  rules,
  title,
}: ColumnProps): ReactElement | null => {
  return (
    <div style={{ border: "1px solid red", padding: 15 }}>
      <h3>{title}</h3>
      {cards.map((card) => (
        <Card key={card.title} {...card} />
      ))}
    </div>
  );
};
