import { PropertyType, VersionedUri } from "@blockprotocol/type-system";
import { faList } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@hashintel/hash-design-system";
import { OwnedById } from "@hashintel/hash-shared/types";
import {
  Box,
  Checkbox,
  checkboxClasses,
  Collapse,
  svgIconClasses,
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TableRow,
} from "@mui/material";
import { bindTrigger, usePopupState } from "material-ui-popup-state/hooks";
import {
  ReactNode,
  useCallback,
  useEffect,
  useId,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  Controller,
  useFieldArray,
  useFormContext,
  useWatch,
} from "react-hook-form";

import { useBlockProtocolCreatePropertyType } from "../../../../../../components/hooks/block-protocol-functions/ontology/use-block-protocol-create-property-type";
import { useBlockProtocolUpdatePropertyType } from "../../../../../../components/hooks/block-protocol-functions/ontology/use-block-protocol-update-property-type";
import { StyledPlusCircleIcon } from "../../../../shared/styled-plus-circle-icon";
import { useRouteNamespace } from "../../../../shared/use-route-namespace";
import { EntityTypeEditorForm } from "../shared/form-types";
import {
  usePropertyTypes,
  useRefetchPropertyTypes,
} from "../shared/property-types-context";
import { getPropertyTypeSchema } from "./property-list-card/get-property-type-schema";
import { PropertyExpectedValues } from "./property-list-card/property-expected-values";
import { PropertyTitleCell } from "./property-list-card/property-title-cell";
import { PropertyTypeForm } from "./property-list-card/property-type-form";
import { propertyTypeToFormDataExpectedValues } from "./property-list-card/property-type-to-form-data-expected-values";
import { PropertyTypeFormValues } from "./property-list-card/shared/property-type-form-values";
import { EmptyListCard } from "./shared/empty-list-card";
import {
  EntityTypeTable,
  EntityTypeTableButtonRow,
  EntityTypeTableCenteredCell,
  EntityTypeTableHeaderRow,
  EntityTypeTableRow,
} from "./shared/entity-type-table";
import { InsertTypeRow, InsertTypeRowProps } from "./shared/insert-type-row";
import {
  MULTIPLE_VALUES_CELL_WIDTH,
  MultipleValuesCell,
} from "./shared/multiple-values-cell";
import { QuestionIcon } from "./shared/question-icon";
import { TypeFormModal } from "./shared/type-form";
import { TypeMenuCell } from "./shared/type-menu-cell";
import { useStateCallback } from "./shared/use-state-callback";

const CollapsibleTableRow = ({
  expanded,
  depth,
  lineHeight,
  children,
}: {
  expanded: boolean;
  depth: number;
  lineHeight: number;
  children: ReactNode;
}) => {
  return (
    <TableRow>
      <TableCell colSpan={12} sx={{ p: "0 !important", position: "relative" }}>
        <Collapse
          in={expanded}
          sx={{
            position: "relative",
            top: `-${lineHeight}px`,
            mb: `-${lineHeight}px`,
          }}
          appear
        >
          <Box
            sx={{
              position: "absolute",
              height: lineHeight,
              width: "1px",
              left: `${13.4 + 20 * depth}px`,
              background: ({ palette }) => palette.gray[30],
              zIndex: 1,
            }}
          />

          <Table sx={{ mt: `${lineHeight}px` }}>
            <TableBody
              sx={{
                "::before": {
                  height: 0,
                },
              }}
            >
              {children}
            </TableBody>
          </Table>
        </Collapse>
      </TableCell>
    </TableRow>
  );
};

const REQUIRED_CELL_WIDTH = 100;

const PropertyRow = ({
  property,
  isArray,
  isRequired,
  depth = 0,
  lines = [],
  allowArraysTableCell,
  requiredTableCell,
  menuTableCell,
}: {
  property: PropertyType;
  isArray: boolean;
  isRequired?: boolean;
  depth?: number;
  lines?: boolean[];
  allowArraysTableCell?: ReactNode;
  requiredTableCell?: ReactNode;
  menuTableCell?: ReactNode;
}) => {
  const propertyTypes = usePropertyTypes();

  const [expanded, setExpanded] = useState(true);

  const mainRef = useRef<HTMLTableRowElement | null>(null);
  const [lineHeight, setLineHeight] = useState(0);

  const [animatingOutExpectedValue, setAnimatingOutExpectedValue] =
    useState(false);
  const [selectedExpectedValueIndex, setSelectedExpectedValueIndex] =
    useState(-1);

  const children = useMemo(() => {
    const selectedProperty = property.oneOf[selectedExpectedValueIndex]
      ? property.oneOf[selectedExpectedValueIndex]
      : null;

    const selectedObjectProperties =
      selectedProperty && "properties" in selectedProperty
        ? selectedProperty.properties
        : undefined;

    return selectedObjectProperties
      ? Object.entries(selectedObjectProperties).reduce(
          (
            childrenArray: ({
              array: boolean;
              required: boolean;
            } & PropertyType)[],
            [propertyId, ref],
          ) => {
            const $ref = "items" in ref ? ref.items.$ref : ref.$ref;
            const propertyType = propertyTypes?.[$ref];

            if (propertyType) {
              const array = "type" in ref;
              const required = Boolean(
                selectedProperty &&
                  "required" in selectedProperty &&
                  selectedProperty.required?.includes(propertyId),
              );
              return [...childrenArray, { ...propertyType, array, required }];
            }

            return childrenArray;
          },
          [],
        )
      : [];
  }, [selectedExpectedValueIndex, property.oneOf, propertyTypes]);

  const handleResize = () => {
    if (mainRef.current) {
      setLineHeight(mainRef.current.offsetHeight * 0.5 - 8);
    }
  };

  useEffect(() => {
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  return (
    <>
      <EntityTypeTableRow
        ref={(row: HTMLTableRowElement | null) => {
          if (row) {
            mainRef.current = row;
            handleResize();
          }
        }}
      >
        <PropertyTitleCell
          property={property}
          array={isArray}
          depth={depth}
          lines={lines}
          expanded={children.length ? expanded : undefined}
          setExpanded={setExpanded}
        />

        <TableCell>
          <PropertyExpectedValues
            property={property}
            selectedExpectedValueIndex={selectedExpectedValueIndex}
            setSelectedExpectedValueIndex={setSelectedExpectedValueIndex}
            setAnimatingOutExpectedValue={setAnimatingOutExpectedValue}
          />
        </TableCell>

        {allowArraysTableCell ?? (
          <EntityTypeTableCenteredCell width={MULTIPLE_VALUES_CELL_WIDTH}>
            <Checkbox
              disabled
              checked={isArray}
              sx={{
                pr: 1,
                [`.${svgIconClasses.root}`]: {
                  color: "inherit",
                },
                [`&.${checkboxClasses.checked}.${checkboxClasses.disabled}`]: {
                  color: ({ palette }) => `${palette.blue[30]} !important`,
                },
              }}
            />
          </EntityTypeTableCenteredCell>
        )}

        {requiredTableCell ?? (
          <EntityTypeTableCenteredCell width={REQUIRED_CELL_WIDTH}>
            <Checkbox
              disabled
              checked={isRequired}
              sx={{
                [`.${svgIconClasses.root}`]: {
                  color: "inherit",
                },
                [`&.${checkboxClasses.checked}.${checkboxClasses.disabled}`]: {
                  color: ({ palette }) => `${palette.blue[30]} !important`,
                },
              }}
            />
          </EntityTypeTableCenteredCell>
        )}

        {menuTableCell ?? (
          <TypeMenuCell
            typeId={property.$id}
            variant="property"
            canEdit={false}
            canRemove={false}
          />
        )}
      </EntityTypeTableRow>

      {children.length ? (
        <CollapsibleTableRow
          expanded={expanded && !animatingOutExpectedValue}
          depth={depth}
          lineHeight={lineHeight}
        >
          {children.map((prop, pos) => (
            <PropertyRow
              key={prop.$id}
              property={prop}
              depth={depth + 1}
              lines={[...lines, pos !== children.length - 1]}
              isArray={prop.array}
              isRequired={prop.required}
            />
          ))}
        </CollapsibleTableRow>
      ) : null}
    </>
  );
};

export const PropertyTypeRow = ({
  propertyIndex,
  onRemove,
  onUpdateVersion,
}: {
  propertyIndex: number;
  onRemove: () => void;
  onUpdateVersion: (nextId: VersionedUri) => void;
}) => {
  const { control } = useFormContext<EntityTypeEditorForm>();

  const [$id, array] = useWatch({
    control,
    name: [
      `properties.${propertyIndex}.$id`,
      `properties.${propertyIndex}.array`,
    ],
  });

  const editModalId = useId();
  const editModalPopupState = usePopupState({
    variant: "popover",
    popupId: `edit-property-type-modal-${editModalId}`,
  });

  const { updatePropertyType } = useBlockProtocolUpdatePropertyType();
  const refetchPropertyTypes = useRefetchPropertyTypes();
  const onUpdateVersionRef = useRef(onUpdateVersion);
  useLayoutEffect(() => {
    onUpdateVersionRef.current = onUpdateVersion;
  });

  const propertyTypes = usePropertyTypes();
  const property = propertyTypes?.[$id];

  const getDefaultValues = useCallback(() => {
    if (!property) {
      throw new Error("Missing property type");
    }

    const [expectedValues, flattenedCustomExpectedValueList] =
      propertyTypeToFormDataExpectedValues(property);

    return {
      name: property.title,
      description: property.description,
      expectedValues,
      flattenedCustomExpectedValueList,
    };
  }, [property]);

  if (!property) {
    if (propertyTypes) {
      throw new Error("Missing property type");
    }

    return null;
  }

  return (
    <>
      <PropertyRow
        property={property}
        isArray={array}
        allowArraysTableCell={
          <MultipleValuesCell index={propertyIndex} variant="property" />
        }
        requiredTableCell={
          <EntityTypeTableCenteredCell width={REQUIRED_CELL_WIDTH}>
            <Controller
              render={({ field: { value, ...field } }) => (
                <Checkbox {...field} checked={value} />
              )}
              control={control}
              name={`properties.${propertyIndex}.required`}
            />
          </EntityTypeTableCenteredCell>
        }
        menuTableCell={
          <TypeMenuCell
            editButtonProps={bindTrigger(editModalPopupState)}
            onRemove={onRemove}
            typeId={property.$id}
            variant="property"
          />
        }
      />

      <TypeFormModal
        as={PropertyTypeForm}
        popupState={editModalPopupState}
        modalTitle={<>Edit Property Type</>}
        onSubmit={async (data) => {
          const res = await updatePropertyType({
            data: {
              propertyTypeId: $id,
              propertyType: getPropertyTypeSchema(data),
            },
          });

          if (!res.data) {
            throw new Error("Failed to update property type");
          }

          await refetchPropertyTypes?.();

          onUpdateVersionRef.current(res.data.schema.$id);

          editModalPopupState.close();
        }}
        submitButtonProps={{ children: <>Edit property type</> }}
        disabledFields={["name"]}
        getDefaultValues={getDefaultValues}
      />
    </>
  );
};

const InsertPropertyRow = (
  props: Omit<InsertTypeRowProps<PropertyType>, "options" | "variant">,
) => {
  const { control } = useFormContext<EntityTypeEditorForm>();
  const properties = useWatch({ control, name: "properties" });

  const propertyTypesObj = usePropertyTypes();
  const propertyTypes = Object.values(propertyTypesObj ?? {});

  // @todo make more efficient
  const filteredPropertyTypes = propertyTypes.filter(
    (type) =>
      !properties.some((includedProperty) => includedProperty.$id === type.$id),
  );

  return (
    <InsertTypeRow
      {...props}
      options={filteredPropertyTypes}
      variant="property"
    />
  );
};

export const PropertyListCard = () => {
  const { control, getValues, setValue } =
    useFormContext<EntityTypeEditorForm>();
  const { fields, append, remove } = useFieldArray({
    control,
    name: "properties",
  });

  const [addingNewProperty, setAddingNewProperty] = useStateCallback(false);
  const [searchText, setSearchText] = useState("");
  const addingNewPropertyRef = useRef<HTMLInputElement>(null);

  const cancelAddingNewProperty = () => {
    setAddingNewProperty(false);
    setSearchText("");
  };

  const { routeNamespace } = useRouteNamespace();
  const { createPropertyType } = useBlockProtocolCreatePropertyType(
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- @todo improve logic or types to remove this comment
    (routeNamespace?.accountId as OwnedById) ?? null,
  );

  const refetchPropertyTypes = useRefetchPropertyTypes();
  const modalTooltipId = useId();
  const createModalPopupState = usePopupState({
    variant: "popover",
    popupId: `createProperty-${modalTooltipId}`,
  });

  const handleAddPropertyType = (propertyType: PropertyType) => {
    cancelAddingNewProperty();
    if (!getValues("properties").some(({ $id }) => $id === propertyType.$id)) {
      append({
        $id: propertyType.$id,
        required: false,
        array: false,
        minValue: 0,
        maxValue: 1,
        infinity: true,
      });
    }
  };

  const handleSubmit = async (data: PropertyTypeFormValues) => {
    const res = await createPropertyType({
      data: {
        propertyType: getPropertyTypeSchema(data),
      },
    });

    if (res.errors?.length || !res.data) {
      // @todo handle this
      throw new Error("Could not create");
    }

    await refetchPropertyTypes?.();

    handleAddPropertyType(res.data.schema);
  };

  if (!addingNewProperty && fields.length === 0) {
    return (
      <EmptyListCard
        onClick={() => {
          setAddingNewProperty(true, () => {
            addingNewPropertyRef.current?.focus();
          });
        }}
        icon={<FontAwesomeIcon icon={faList} />}
        headline={<>Add a property</>}
        description={
          <>
            Properties store individual pieces of information about some aspect
            of an entity
          </>
        }
        subDescription={
          <>
            e.g. a <strong>person</strong> entity might have a{" "}
            <strong>date of birth</strong> property which expects a{" "}
            <strong>date</strong>
          </>
        }
      />
    );
  }

  return (
    <EntityTypeTable>
      <TableHead>
        <EntityTypeTableHeaderRow>
          <TableCell>Property name</TableCell>
          <TableCell>Expected values</TableCell>
          <EntityTypeTableCenteredCell>
            Allow arrays{" "}
            <QuestionIcon
              tooltip={
                <>
                  Allowing arrays permits the entry of more than one value for a
                  given property
                </>
              }
            />
          </EntityTypeTableCenteredCell>
          <EntityTypeTableCenteredCell>Required</EntityTypeTableCenteredCell>
          <TableCell />
        </EntityTypeTableHeaderRow>
      </TableHead>
      <TableBody>
        {fields.map((type, index) => (
          <PropertyTypeRow
            key={type.id}
            propertyIndex={index}
            onRemove={() => {
              remove(index);
            }}
            onUpdateVersion={(nextId) => {
              setValue(`properties.${index}.$id`, nextId, {
                shouldDirty: true,
              });
            }}
          />
        ))}
      </TableBody>
      <TableFooter>
        {addingNewProperty ? (
          <>
            <InsertPropertyRow
              inputRef={addingNewPropertyRef}
              onCancel={cancelAddingNewProperty}
              onAdd={handleAddPropertyType}
              createModalPopupState={createModalPopupState}
              searchText={searchText}
              onSearchTextChange={setSearchText}
            />
            <TypeFormModal
              as={PropertyTypeForm}
              modalTitle={
                <>
                  Create new property type
                  <QuestionIcon
                    sx={{
                      ml: 1.25,
                    }}
                    tooltip={
                      <>
                        You should only create a new property type if you can't
                        find an existing one which corresponds to the
                        information you're trying to capture.
                      </>
                    }
                  />
                </>
              }
              popupState={createModalPopupState}
              onSubmit={handleSubmit}
              submitButtonProps={{ children: <>Create new property type</> }}
              getDefaultValues={() => ({
                expectedValues: [],
                ...(searchText.length ? { name: searchText } : {}),
              })}
            />
          </>
        ) : (
          <EntityTypeTableButtonRow
            icon={<StyledPlusCircleIcon />}
            onClick={() => {
              setAddingNewProperty(true, () => {
                addingNewPropertyRef.current?.focus();
              });
            }}
          >
            Add a property
          </EntityTypeTableButtonRow>
        )}
      </TableFooter>
    </EntityTypeTable>
  );
};
