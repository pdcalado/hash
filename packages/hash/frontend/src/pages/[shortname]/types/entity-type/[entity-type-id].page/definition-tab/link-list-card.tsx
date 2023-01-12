import { EntityType, VersionedUri } from "@blockprotocol/type-system";
import { faAsterisk } from "@fortawesome/free-solid-svg-icons";
import { Chip, FontAwesomeIcon } from "@hashintel/hash-design-system";
import { OwnedById } from "@hashintel/hash-shared/types";
import { linkEntityTypeUri } from "@hashintel/hash-subgraph";
import { getEntityTypeById } from "@hashintel/hash-subgraph/src/stdlib/element/entity-type";
import {
  ClickAwayListener,
  Stack,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  tableRowClasses,
} from "@mui/material";
import { Box } from "@mui/system";
import { bindTrigger, usePopupState } from "material-ui-popup-state/hooks";
import { useId, useLayoutEffect, useRef, useState } from "react";
import { useFieldArray, useFormContext, useWatch } from "react-hook-form";

import { useBlockProtocolCreateEntityType } from "../../../../../../components/hooks/block-protocol-functions/ontology/use-block-protocol-create-entity-type";
import { useBlockProtocolGetEntityType } from "../../../../../../components/hooks/block-protocol-functions/ontology/use-block-protocol-get-entity-type";
import { useBlockProtocolUpdateEntityType } from "../../../../../../components/hooks/block-protocol-functions/ontology/use-block-protocol-update-entity-type";
import {
  useEntityTypes,
  useFetchEntityTypes,
  useLinkEntityTypes,
  useLinkEntityTypesOptional,
} from "../../../../../../shared/entity-types-context/hooks";
import { LinkIcon } from "../../../../../../shared/icons/link";
import { HashSelectorAutocomplete } from "../../../../shared/hash-selector-autocomplete";
import {
  addPopperPositionClassPopperModifier,
  popperPlacementInputNoRadius,
  popperPlacementPopperNoRadius,
} from "../../../../shared/popper-placement-modifier";
import { StyledPlusCircleIcon } from "../../../../shared/styled-plus-circle-icon";
import { useRouteNamespace } from "../../../../shared/use-route-namespace";
import { EntityTypeEditorForm } from "../shared/form-types";
import { EmptyListCard } from "./shared/empty-list-card";
import {
  EntityTypeTable,
  EntityTypeTableButtonRow,
  EntityTypeTableCenteredCell,
  EntityTypeTableHeaderRow,
  EntityTypeTableRow,
  EntityTypeTableTitleCellText,
  rowBackground,
} from "./shared/entity-type-table";
import { InsertTypeRow, InsertTypeRowProps } from "./shared/insert-type-row";
import { MultipleValuesCell } from "./shared/multiple-values-cell";
import { QuestionIcon } from "./shared/question-icon";
import {
  generateInitialTypeUri,
  TypeForm,
  TypeFormDefaults,
  TypeFormModal,
  TypeFormProps,
  useGenerateTypeBaseUri,
} from "./shared/type-form";
import { TYPE_MENU_CELL_WIDTH, TypeMenuCell } from "./shared/type-menu-cell";
import { useStateCallback } from "./shared/use-state-callback";

const formDataToEntityType = (data: TypeFormDefaults) => ({
  type: "object" as const,
  kind: "entityType" as const,
  title: data.name,
  description: data.description,
  allOf: [
    {
      $ref: linkEntityTypeUri,
    },
  ],
  properties: {},
});

export const LinkTypeForm = (props: TypeFormProps) => {
  const { getEntityType } = useBlockProtocolGetEntityType();
  const generateTypeBaseUri = useGenerateTypeBaseUri("entity-type");

  const nameExists = async (name: string) => {
    const entityTypeId = generateInitialTypeUri(generateTypeBaseUri(name));

    const res = await getEntityType({
      data: {
        entityTypeId,
        graphResolveDepths: {
          constrainsValuesOn: { outgoing: 0 },
          constrainsPropertiesOn: { outgoing: 0 },
        },
      },
    });

    if (!res.data) {
      // @todo consider non-crash error handling
      throw new Error("Unable to check whether name is available");
    }

    return !!getEntityTypeById(res.data, entityTypeId);
  };

  return <TypeForm nameExists={nameExists} {...props} />;
};

const LinkTypeRow = ({
  linkIndex,
  onRemove,
  onUpdateVersion,
}: {
  linkIndex: number;
  onRemove: () => void;
  onUpdateVersion: (nextId: VersionedUri) => void;
}) => {
  const { control, setValue } = useFormContext<EntityTypeEditorForm>();

  const entityTypeSelectorPopupId = useId();
  // @todo replace with state
  const entityTypeSelectorPopupState = usePopupState({
    variant: "popper",
    popupId: entityTypeSelectorPopupId,
  });

  const linkTypes = useLinkEntityTypes();
  const entityTypes = useEntityTypes();
  const linkId = useWatch({
    control,
    name: `links.${linkIndex}.$id`,
  });

  const chosenEntityTypes = useWatch({
    control,
    name: `links.${linkIndex}.entityTypes`,
  });

  const popupId = useId();
  const menuPopupState = usePopupState({
    variant: "popover",
    popupId: `property-menu-${popupId}`,
  });

  const editModalPopupId = useId();
  const editModalPopupState = usePopupState({
    variant: "popover",
    popupId: `editLink-${editModalPopupId}`,
  });

  const { updateEntityType } = useBlockProtocolUpdateEntityType();
  const refetchEntityTypes = useFetchEntityTypes();
  const onUpdateVersionRef = useRef(onUpdateVersion);
  useLayoutEffect(() => {
    onUpdateVersionRef.current = onUpdateVersion;
  });

  const handleSubmit = async (data: TypeFormDefaults) => {
    const res = await updateEntityType({
      data: {
        entityTypeId: linkId,
        entityType: formDataToEntityType(data),
      },
    });

    if (!res.data) {
      throw new Error("Failed to update property type");
    }

    await refetchEntityTypes();

    onUpdateVersionRef.current(res.data.schema.$id);

    editModalPopupState.close();
  };

  const link = linkTypes[linkId];

  if (!link) {
    throw new Error("Missing link");
  }

  const entityTypeSchemas = Object.values(entityTypes).map(
    (type) => type.schema,
  );

  const chosenEntityTypeSchemas = entityTypeSchemas.filter((schema) =>
    chosenEntityTypes.includes(schema.$id),
  );

  return (
    <>
      <EntityTypeTableRow>
        <TableCell>
          <EntityTypeTableTitleCellText>
            {link.schema.title}
          </EntityTypeTableTitleCellText>
        </TableCell>
        <TableCell
          sx={(theme) => ({
            [entityTypeSelectorPopupState.isOpen ? "& > *" : "&:hover > *"]: {
              boxShadow: theme.boxShadows.xs,
              borderColor: `${theme.palette.gray[30]} !important`,
              backgroundColor: "white",

              "&:after": {
                display: "none",
              },
            },

            "&, *": {
              cursor: "pointer",
            },
          })}
        >
          <Box sx={{ position: "relative" }}>
            <Stack
              direction="row"
              flexWrap="wrap"
              sx={[
                {
                  border: 1,
                  borderColor: "transparent",
                  borderRadius: 1.5,
                  p: 0.5,
                  width: 1,
                  userSelect: "none",
                  minWidth: 200,
                  minHeight: 42,
                  left: -7,
                  overflow: "hidden",
                  "&:after": {
                    position: "absolute",
                    top: 0,
                    right: 0,
                    width: "20px",
                    height: "100%",
                    zIndex: 1,
                    display: "block",
                    content: `""`,
                    background: "linear-gradient(to right, transparent, white)",
                  },
                  [`.${tableRowClasses.root}:hover &:after`]: (theme) => ({
                    background: `linear-gradient(to right, transparent, ${rowBackground(
                      theme,
                    )})`,
                  }),
                },
                ...(entityTypeSelectorPopupState.isOpen
                  ? [popperPlacementInputNoRadius]
                  : []),
              ]}
              {...bindTrigger(entityTypeSelectorPopupState)}
            >
              {chosenEntityTypes.length ? (
                chosenEntityTypes.map((entityTypeId) => {
                  const type = entityTypes[entityTypeId];

                  if (!type) {
                    throw new Error("Entity type missing in links table");
                  }

                  return (
                    <Chip
                      sx={{ m: 0.25 }}
                      color="blue"
                      label={
                        <Stack
                          direction="row"
                          spacing={0.75}
                          fontSize={14}
                          alignItems="center"
                        >
                          <FontAwesomeIcon
                            icon={faAsterisk}
                            sx={{ fontSize: "inherit" }}
                          />
                          <Box component="span">{type.schema.title}</Box>
                        </Stack>
                      }
                      key={type.schema.$id}
                    />
                  );
                })
              ) : (
                <Chip
                  color="blue"
                  variant="outlined"
                  label={
                    <Stack
                      direction="row"
                      spacing={0.75}
                      fontSize={14}
                      alignItems="center"
                    >
                      <FontAwesomeIcon
                        icon={faAsterisk}
                        sx={{ fontSize: "inherit" }}
                      />
                      <Box component="span">Anything</Box>
                    </Stack>
                  }
                />
              )}
            </Stack>
            {entityTypeSelectorPopupState.isOpen ? (
              <ClickAwayListener
                onClickAway={entityTypeSelectorPopupState.close}
              >
                <Box
                  onClick={(evt) => {
                    evt.stopPropagation();
                    evt.preventDefault();
                  }}
                  sx={{
                    position: "absolute",
                    width: "100%",
                    minHeight: "100%",
                    top: 0,
                    left: 0,
                    zIndex: -1,
                  }}
                >
                  <HashSelectorAutocomplete
                    multiple
                    sx={[popperPlacementPopperNoRadius, { width: "100%" }]}
                    open
                    onChange={(_, chosenTypes) => {
                      setValue(
                        `links.${linkIndex}.entityTypes`,
                        chosenTypes.map((type) => type.$id),
                      );
                    }}
                    options={entityTypeSchemas}
                    optionToRenderData={({ $id, title, description }) => ({
                      $id,
                      title,
                      description,
                    })}
                    dropdownProps={{
                      query: "",
                      createButtonProps: null,
                      variant: "entityType",
                    }}
                    renderTags={() => <Box />}
                    value={chosenEntityTypeSchemas}
                  />
                </Box>
              </ClickAwayListener>
            ) : null}
          </Box>
        </TableCell>
        <MultipleValuesCell index={linkIndex} variant="link" />
        <TypeMenuCell
          typeId={linkId}
          editButtonProps={bindTrigger(editModalPopupState)}
          popupState={menuPopupState}
          variant="link"
          onRemove={onRemove}
        />
      </EntityTypeTableRow>
      <TypeFormModal
        as={LinkTypeForm}
        popupState={editModalPopupState}
        modalTitle={<>Edit link</>}
        onSubmit={handleSubmit}
        submitButtonProps={{ children: <>Edit link</> }}
        disabledFields={["name"]}
        getDefaultValues={() => ({
          name: link.schema.title,
          description: link.schema.description,
        })}
      />
    </>
  );
};

const InsertLinkRow = (
  props: Omit<
    InsertTypeRowProps<EntityType>,
    "options" | "variant" | "createButtonProps"
  >,
) => {
  const { control } = useFormContext<EntityTypeEditorForm>();
  const links = useWatch({ control, name: "links" });

  const linkTypes = Object.values(useLinkEntityTypes()).map(
    (link) => link.schema,
  );

  // @todo make more efficient
  const filteredLinkTypes = linkTypes.filter(
    (type) => !links.some((includedLink) => includedLink.$id === type.$id),
  );

  return (
    <InsertTypeRow {...props} options={filteredLinkTypes} variant="link" />
  );
};

export const LinkListCard = () => {
  const { control, setValue } = useFormContext<EntityTypeEditorForm>();
  const { fields, append, remove } = useFieldArray({ control, name: "links" });
  const linkEntityTypes = useLinkEntityTypesOptional();
  const [addingNewLink, setAddingNewLink] = useStateCallback(false);
  const addingNewLinkRef = useRef<HTMLInputElement>(null);
  const [searchText, setSearchText] = useState("");
  const modalId = useId();
  const createModalPopupState = usePopupState({
    variant: "popover",
    popupId: `createLink-${modalId}`,
  });

  const { routeNamespace } = useRouteNamespace();
  const refetchEntityTypes = useFetchEntityTypes();
  const { createEntityType } = useBlockProtocolCreateEntityType(
    routeNamespace?.accountId as OwnedById,
  );

  const cancelAddingNewLink = () => {
    setAddingNewLink(false);
    setSearchText("");
  };

  const handleAddEntityType = (link: EntityType) => {
    cancelAddingNewLink();
    append(
      {
        $id: link.$id,
        entityTypes: [],
        minValue: 0,
        maxValue: 1,
        infinity: true,
        array: true,
      },
      { shouldFocus: false },
    );
  };

  const handleSubmit = async (data: TypeFormDefaults) => {
    const res = await createEntityType({
      data: {
        entityType: formDataToEntityType(data),
      },
    });

    if (res.errors?.length || !res.data) {
      // @todo handle this
      throw new Error("Could not create");
    }

    await refetchEntityTypes();

    handleAddEntityType(res.data.schema);
  };

  // @todo loading state
  if (!linkEntityTypes) {
    return null;
  }

  if (!addingNewLink && fields.length === 0) {
    return (
      <EmptyListCard
        onClick={() => {
          setAddingNewLink(true, () => {
            addingNewLinkRef.current?.focus();
          });
        }}
        icon={<LinkIcon />}
        headline={<>Add a link</>}
        description={
          <>
            Links contain information about connections or relationships between
            different entities
          </>
        }
        subDescription={
          <>
            e.g. a <strong>company</strong> entity might have a{" "}
            <strong>CEO</strong> link which points to a <strong>person</strong>{" "}
            entity
          </>
        }
      />
    );
  }

  return (
    <EntityTypeTable>
      <TableHead>
        <EntityTypeTableHeaderRow>
          <TableCell width={260}>Link name</TableCell>
          <TableCell sx={{ minWidth: 262 }}>
            Expected entity types{" "}
            <QuestionIcon tooltip="When specified, only entities whose types are listed in this column will be able to be associated with a link" />
          </TableCell>
          <EntityTypeTableCenteredCell width={200}>
            Allowed number of links{" "}
            <QuestionIcon tooltip="Require entities to specify a minimum or maximum number of links. A minimum value of 1 or more means that a link is required." />
          </EntityTypeTableCenteredCell>
          <TableCell width={TYPE_MENU_CELL_WIDTH} />
        </EntityTypeTableHeaderRow>
      </TableHead>
      <TableBody>
        {fields.map((type, index) => (
          <LinkTypeRow
            key={type.id}
            linkIndex={index}
            onRemove={() => {
              remove(index);
            }}
            onUpdateVersion={(nextId) => {
              setValue(`links.${index}.$id`, nextId, {
                shouldDirty: true,
              });
            }}
          />
        ))}
      </TableBody>
      <TableFooter>
        {addingNewLink ? (
          <>
            <InsertLinkRow
              inputRef={addingNewLinkRef}
              onCancel={cancelAddingNewLink}
              onAdd={handleAddEntityType}
              searchText={searchText}
              onSearchTextChange={setSearchText}
              createModalPopupState={createModalPopupState}
            />
            <TypeFormModal
              as={LinkTypeForm}
              popupState={createModalPopupState}
              modalTitle={
                <>
                  Create new link
                  <QuestionIcon
                    sx={{
                      ml: 1.25,
                    }}
                    tooltip={
                      <>
                        You should only create a new link type if you can't find
                        an existing one which corresponds to the relationship
                        you're trying to capture.
                      </>
                    }
                  />
                </>
              }
              onSubmit={handleSubmit}
              submitButtonProps={{ children: <>Create new link</> }}
              getDefaultValues={() =>
                searchText.length ? { name: searchText } : {}
              }
            />
          </>
        ) : (
          <EntityTypeTableButtonRow
            icon={<StyledPlusCircleIcon />}
            onClick={() => {
              setAddingNewLink(true, () => {
                addingNewLinkRef.current?.focus();
              });
            }}
          >
            Add a link
          </EntityTypeTableButtonRow>
        )}
      </TableFooter>
    </EntityTypeTable>
  );
};
