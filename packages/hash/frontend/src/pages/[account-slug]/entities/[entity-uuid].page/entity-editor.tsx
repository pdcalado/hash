import {
  Entity,
  EntityId,
  extractEntityUuidFromEntityId,
  Subgraph,
  SubgraphRootTypes,
} from "@hashintel/hash-subgraph";
import { PropertyTypeWithoutId } from "@hashintel/hash-shared/graphql/types";
import { useContext, useState } from "react";
import { VersionedUri } from "@blockprotocol/type-system";
import { EntityEditorContextProvider } from "./entity-editor/entity-editor-context";
import { LinksSection } from "./entity-editor/links-section";
import { PeersSection } from "./entity-editor/peers-section";
import { PropertiesSection } from "./entity-editor/properties-section";
import { TypesSection } from "./entity-editor/types-section";
import { useBlockProtocolFunctionsWithOntology } from "../../../type-editor/blockprotocol-ontology-functions-hook";
import { WorkspaceContext } from "../../../shared/workspace-context";

export interface EntityEditorProps {
  entitySubgraph: Subgraph<SubgraphRootTypes["entity"]>;
  setEntity: (entity: Entity | undefined) => void;
  refetch: () => Promise<void>;
}

const useCreateDemoEntityType = (): [() => Promise<void>, boolean] => {
  const { activeWorkspaceAccountId } = useContext(WorkspaceContext);

  const { createPropertyType, createEntityType, createEntity } =
    useBlockProtocolFunctionsWithOntology(activeWorkspaceAccountId ?? null);

  const [loading, setLoading] = useState(false);

  const func = async () => {
    const rand = Date.now().toString(36);
    try {
      setLoading(true);
      const ebu = (url: string) => {
        return url.replace(/v\/\d+/, "");
      };
      const createPropertyTypeAndGetId = async (
        propertyType: PropertyTypeWithoutId,
      ) => {
        let id;
        try {
          const response = await createPropertyType({
            data: {
              propertyType,
            },
          });
          id = response.data?.schema.$id;
        } catch (err: any) {
          // THIS IS SUPER HACKY
          if (err.message) {
            if (
              // eslint-disable-next-line @typescript-eslint/no-unsafe-call
              err.message
                .toString()
                .includes("with the same URI already exists.")
            ) {
              // eslint-disable-next-line @typescript-eslint/no-unsafe-call
              return [...err.message.toString().matchAll(/\[URI=(.*)]/g)][0][1];
            }
          }
          throw err;
        }
        return id;
      };
      const createUserAndGetEntityId = async (name: string) => {
        const res = await createEntity({
          data: {
            entityTypeId:
              "http://localhost:3000/@system-user/types/entity-type/user/v/1",
            properties: {
              "http://localhost:3000/@system-user/types/property-type/preferred-name/":
                name,
            },
          },
        });

        return res.data?.metadata.editionId.baseId as EntityId;
      };
      const createLinkEntityTypeAndGetId = async (title: string) => {
        const res = await createEntityType({
          data: {
            entityType: {
              kind: "entityType",
              title: `${title} ${rand}`,
              properties: {},
              type: "object",
              allOf: [
                {
                  $ref: "https://blockprotocol.org/@blockprotocol/types/entity-type/link/v/1",
                },
              ],
            },
          },
        });

        return res.data?.schema.$id as VersionedUri;
      };

      const textArray = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Array [Text]",
        oneOf: [
          {
            type: "array",
            items: {
              oneOf: [
                {
                  $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
                },
              ],
            },
          },
        ],
      });

      const numberArray = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Array [Number]",
        oneOf: [
          {
            type: "array",
            items: {
              oneOf: [
                {
                  $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/number/v/1",
                },
              ],
            },
          },
        ],
      });

      const textOrNumberArray = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Array [Number or Text]",
        oneOf: [
          {
            type: "array",
            items: {
              oneOf: [
                {
                  $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
                },
                {
                  $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/number/v/1",
                },
              ],
            },
          },
        ],
      });

      const nameId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Name",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const numberOrBooleanOrTextId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Mixed Type",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/number/v/1",
          },
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/boolean/v/1",
          },
        ],
      });
      const marriedId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Is Married",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/boolean/v/1",
          },
        ],
      });
      const ageId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Age",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/number/v/1",
          },
        ],
      });
      const favoriteFilmId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Favorite Film",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const favoriteSongId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Favorite Song",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });

      const hobbyId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Hobby",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const fifaId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Fifa",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const godOfWarId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "God Of War",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const lolId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "League of Legends",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const dotaId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Dota 2",
        oneOf: [
          {
            $ref: "https://blockprotocol.org/@blockprotocol/types/data-type/text/v/1",
          },
        ],
      });
      const computerId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "PC Gaming",
        oneOf: [
          {
            type: "object",
            properties: {
              [ebu(lolId)]: {
                $ref: lolId,
              },
              [ebu(dotaId)]: {
                $ref: dotaId,
              },
            },
          },
        ],
      });
      const psId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Console Gaming",
        oneOf: [
          {
            type: "object",
            properties: {
              [ebu(fifaId)]: {
                $ref: fifaId,
              },
              [ebu(godOfWarId)]: {
                $ref: godOfWarId,
              },
            },
          },
        ],
      });
      const gamingId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "Gaming",
        oneOf: [
          {
            type: "object",
            properties: {
              [ebu(psId)]: {
                $ref: psId,
              },
              [ebu(computerId)]: {
                $ref: computerId,
              },
            },
          },
        ],
      });
      const interestsId = await createPropertyTypeAndGetId({
        kind: "propertyType",
        title: "My Interests",
        oneOf: [
          {
            type: "object",
            properties: {
              [ebu(numberArray)]: {
                $ref: numberArray,
              },
              [ebu(favoriteFilmId)]: {
                $ref: favoriteFilmId,
              },
              [ebu(favoriteSongId)]: {
                $ref: favoriteSongId,
              },
              [ebu(hobbyId)]: {
                $ref: hobbyId,
              },
              [ebu(gamingId)]: {
                $ref: gamingId,
              },
            },
          },
        ],
      });

      const properties = {
        [ebu(nameId)]: {
          $ref: nameId,
        },
        [ebu(numberOrBooleanOrTextId)]: {
          $ref: numberOrBooleanOrTextId,
        },
        [ebu(ageId)]: {
          $ref: ageId,
        },
        [ebu(marriedId)]: {
          $ref: marriedId,
        },
        [ebu(interestsId)]: {
          $ref: interestsId,
        },
        [ebu(textArray)]: {
          $ref: textArray,
        },
        [ebu(numberArray)]: {
          $ref: numberArray,
        },
        [ebu(textOrNumberArray)]: {
          $ref: textOrNumberArray,
        },
      };

      const dadLinkTypeId = await createLinkEntityTypeAndGetId("Dad");
      const momLinkTypeId = await createLinkEntityTypeAndGetId("Mom");
      const wifeLinkTypeId = await createLinkEntityTypeAndGetId("Wife");
      const friendsLinkTypeId = await createLinkEntityTypeAndGetId("Friends");

      const myEntityTypeResponse = await createEntityType({
        data: {
          entityType: {
            kind: "entityType",
            type: "object",
            title: `My Entity Type ${rand}`,
            properties,
            links: {
              [dadLinkTypeId]: {
                items: {
                  oneOf: [
                    {
                      $ref: "http://localhost:3000/@system-user/types/entity-type/user/v/1",
                    },
                  ],
                },
                maxItems: 1,
                ordered: false,
                type: "array",
              },
              [momLinkTypeId]: {
                items: {
                  oneOf: [
                    {
                      $ref: "http://localhost:3000/@system-user/types/entity-type/user/v/1",
                    },
                  ],
                },
                maxItems: 1,
                ordered: false,
                type: "array",
              },
              [wifeLinkTypeId]: {
                items: {
                  oneOf: [
                    {
                      $ref: "http://localhost:3000/@system-user/types/entity-type/user/v/1",
                    },
                  ],
                },
                maxItems: 1,
                ordered: false,
                type: "array",
              },
              [friendsLinkTypeId]: {
                items: {
                  oneOf: [
                    {
                      $ref: "http://localhost:3000/@system-user/types/entity-type/user/v/1",
                    },
                  ],
                },
                maxItems: 5,
                ordered: false,
                type: "array",
              },
            },
            required: [],
          },
        },
      });

      const { data: entity } = await createEntity({
        data: {
          entityTypeId: myEntityTypeResponse.data?.schema.$id!,
          properties: {
            [ebu(numberOrBooleanOrTextId)]: "This could be anything",
            [ebu(nameId)]: "Yusuf Kınataş",
            [ebu(ageId)]: 25,
            [ebu(marriedId)]: true,
            [ebu(textArray)]: ["yusuf", "mehmet"],
            [ebu(numberArray)]: [1, 2, 3],
            [ebu(textOrNumberArray)]: ["yusuf", 24],
            [ebu(interestsId)]: {
              [ebu(numberArray)]: [3, 2, 1],
              [ebu(favoriteFilmId)]: "Lord of the Rings",
              [ebu(favoriteSongId)]: "Enter Sandman",
              [ebu(hobbyId)]: "Animes",
              [ebu(gamingId)]: {
                [ebu(computerId)]: {
                  [ebu(dotaId)]: "Good",
                  [ebu(lolId)]: "Bad",
                },
                [ebu(psId)]: {
                  [ebu(fifaId)]: "Good",
                  [ebu(godOfWarId)]: "Bad",
                },
              },
            },
          },
        },
      });

      const entityId = entity?.metadata.editionId.baseId!;

      const dadId = await createUserAndGetEntityId("Jordan");
      const momId = await createUserAndGetEntityId("Melinda");
      const friend1Id = await createUserAndGetEntityId("Yusuf");
      const friend2Id = await createUserAndGetEntityId("Alex");
      const friend3Id = await createUserAndGetEntityId("David");

      await createEntity({
        data: {
          entityTypeId: dadLinkTypeId,
          properties: {},
          linkData: {
            leftEntityId: entityId,
            rightEntityId: dadId,
          },
        },
      });

      await createEntity({
        data: {
          entityTypeId: momLinkTypeId,
          properties: {},
          linkData: {
            leftEntityId: entityId,
            rightEntityId: momId,
          },
        },
      });

      await createEntity({
        data: {
          entityTypeId: friendsLinkTypeId,
          properties: {},
          linkData: {
            leftEntityId: entityId,
            rightEntityId: friend1Id,
          },
        },
      });
      await createEntity({
        data: {
          entityTypeId: friendsLinkTypeId,
          properties: {},
          linkData: {
            leftEntityId: entityId,
            rightEntityId: friend2Id,
          },
        },
      });
      await createEntity({
        data: {
          entityTypeId: friendsLinkTypeId,
          properties: {},
          linkData: {
            leftEntityId: entityId,
            rightEntityId: friend3Id,
          },
        },
      });

      window.open(
        `http://localhost:3000/@alice/entities/${extractEntityUuidFromEntityId(
          entityId,
        )}`,
        "_blank",
      );
    } catch (err: any) {
      // eslint-disable-next-line no-alert
      alert("error");
    } finally {
      setLoading(false);
    }
  };

  return [func, loading];
};

export const EntityEditor = ({
  entitySubgraph,
  setEntity,
  refetch,
}: EntityEditorProps) => {
  const [create, loading] = useCreateDemoEntityType();
  return (
    <EntityEditorContextProvider
      entitySubgraph={entitySubgraph}
      setEntity={setEntity}
      refetch={refetch}
    >
      <button type="button" onClick={create} disabled={loading}>
        {loading ? "Loading" : "Create"}
      </button>
      <TypesSection />

      <PropertiesSection />

      <LinksSection />

      <PeersSection />
    </EntityEditorContextProvider>
  );
};
