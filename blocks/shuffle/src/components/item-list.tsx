import {
  closestCenter,
  DndContext,
  DragOverlay,
  DropAnimation,
  MeasuringStrategy,
  PointerSensor,
  UniqueIdentifier,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  SortableContext,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { List } from "@mui/material";
import React, { FunctionComponent, useRef, useState } from "react";

import { Item } from "../shuffle";
import { Item as ItemComponent } from "./item";
import { SortableItem } from "./sortable-item";

type ItemListProps = {
  list: Item[];
  onReorder: (sourceIndex: number, destinationIndex: number) => void;
  onValueChange: (index: number, value: string) => void;
  onItemBlur: () => void;
  onDelete: (index: number) => void;
  readonly: boolean;
};

const measuringConfig = {
  droppable: {
    strategy: MeasuringStrategy.Always,
  },
};

const findItemIndexById = (list: Item[], id: UniqueIdentifier) =>
  list.findIndex((item) => item.id === id);

const boxShadow =
  "rgba(63, 63, 68, 0.05) 0px 0px 0px 1px, rgba(34, 33, 81, 0.15) 0px 1px 3px 0px";
const draggingBoxShadow =
  "-1px 0 15px 0 rgba(34, 33, 81, 0.01), 0px 15px 15px 0 rgba(34, 33, 81, 0.25)";

export const ItemList: FunctionComponent<ItemListProps> = ({
  list,
  onReorder,
  onValueChange,
  onItemBlur,
  onDelete,
  readonly,
}) => {
  const [activeIndex, setActiveIndex] = useState<number | null>(null);
  const [droppingId, setDroppingId] = useState<UniqueIdentifier | null>(null);

  const sensors = useSensors(useSensor(PointerSensor));

  const dragOverlayRef = useRef<HTMLDivElement | null>(null);

  const activeItem = activeIndex !== null && list[activeIndex];

  const dropAnimationConfig: DropAnimation = {
    keyframes({ transform }) {
      return [
        {
          transform: CSS.Transform.toString(transform.initial),
        },
        {
          transform: CSS.Transform.toString({
            ...transform.final,
            scaleX: 0.95,
            scaleY: 0.95,
          }),
        },
      ];
    },
    duration: 250,
    sideEffects({ active }) {
      setDroppingId(active.id);

      if (dragOverlayRef.current) {
        dragOverlayRef.current.animate(
          [
            {
              boxShadow: draggingBoxShadow,
            },
            { boxShadow },
          ],
          {
            duration: 250,
            easing: "ease",
            fill: "forwards",
          },
        );
      }

      return () => {
        setDroppingId(null);
      };
    },
  };

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragStart={({ active }) =>
        setActiveIndex(findItemIndexById(list, active.id))
      }
      onDragEnd={({ active, over }) => {
        setActiveIndex(null);

        if (over?.id && active.id !== over.id) {
          const sourceIndex = findItemIndexById(list, active.id);
          const destinationIndex = findItemIndexById(list, over.id);
          onReorder(sourceIndex, destinationIndex);
        }
      }}
      onDragCancel={() => setActiveIndex(null)}
      measuring={measuringConfig}
    >
      <SortableContext items={list} strategy={verticalListSortingStrategy}>
        <List>
          {list.map((item, index) => (
            <SortableItem
              key={item.id}
              id={item.id}
              value={item.value}
              isDragging={index === activeIndex || droppingId === item.id}
              onValueChange={(value: string) => onValueChange(index, value)}
              onItemBlur={() => onItemBlur()}
              onDelete={() => onDelete(index)}
              paperStyle={{ boxShadow }}
              readonly={readonly}
              linkedToEntity={!!item.entityId}
            />
          ))}
          <DragOverlay dropAnimation={dropAnimationConfig}>
            {activeItem ? (
              <ItemComponent
                id={activeItem.id}
                value={activeItem.value}
                paperStyle={{
                  boxShadow: draggingBoxShadow,
                  transform: "scale(1.05)",
                  "@keyframes pop": {
                    from: {
                      transform: "scale(1)",
                    },
                    to: {
                      transform: "scale(1.05)",
                    },
                  },
                  animation: `pop 250ms normal ease`,
                }}
                dragOverlay={dragOverlayRef}
                readonly={readonly}
                linkedToEntity={!!activeItem.entityId}
              />
            ) : null}
          </DragOverlay>
        </List>
      </SortableContext>
    </DndContext>
  );
};
