import { Entity } from "@blockprotocol/graph";

export type ImageProperties = {
  url: string;
};

export type OwnerProperties = {
  name: string;
  image: Entity<ImageProperties>;
};

export type CardProperties = {
  title: string;
  description?: string;
  stage?: string;
  assignee?: Entity<OwnerProperties>;
};

export type ColumnRule = {
  key: string;
  value: string;
  rule: "setValueOnEntrance" | "assignToColumnIfValueMet";
};

export type ColumnProperties = {
  title: string;
  rules: ColumnRule[];
  cards: Entity<CardProperties>[];
};

export type GetEntityFn = {
  <T extends Entity>(sourceEntityId: string, path: string): T[] | null;
};
