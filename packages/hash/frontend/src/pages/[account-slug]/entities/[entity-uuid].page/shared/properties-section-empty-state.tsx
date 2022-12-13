import { Chip } from "@hashintel/hash-design-system";
import { Icon } from "../../../../../shared/icons/icon";
import { SectionEmptyState } from "../../../shared/section-empty-state";
import { SectionWrapper } from "../../../shared/section-wrapper";

export const PropertiesSectionEmptyState = () => {
  return (
    <SectionWrapper
      title="Properties"
      titleStartContent={<Chip label="No properties" />}
    >
      <SectionEmptyState
        title="This entity currently has no properties"
        titleIcon={<Icon icon="links" style={{ fontSize: 24 }} />}
        description="Properties contain data about entities, and are inherited from types"
      />
    </SectionWrapper>
  );
};
