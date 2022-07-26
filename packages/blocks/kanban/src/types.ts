export type Owner = {
  name: string;
  avatarUrl?: string;
};

export type Card = {
  title: string;
  description?: string;
  stage?: string;
  assignee?: Owner;
};

export type ColumnRule = {
  key: string;
  value: string;
  rule: "setValueOnEntrance" | "assignToColumnIfValueMet";
};

export type Column = {
  title: string;
  rules: ColumnRule[];
  cards: Card[];
};
