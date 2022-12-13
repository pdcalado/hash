import { Chip } from "@hashintel/hash-design-system";
import { Icon } from "../../../../../shared/icons/icon";
import { SectionEmptyState } from "../../../shared/section-empty-state";
import { SectionWrapper } from "../../../shared/section-wrapper";

export const LinksSectionEmptyState = () => {
  return (
    <SectionWrapper title="Links" titleStartContent={<Chip label="No links" />}>
      <SectionEmptyState
        title="This entity currently has no links"
        titleIcon={<Icon icon="links" style={{ fontSize: 24 }} />}
        description="Links contain information about connections or relationships between different entities"
      />
    </SectionWrapper>
  );
};
