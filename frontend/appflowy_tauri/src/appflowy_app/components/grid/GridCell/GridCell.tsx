import { CellIdentifier } from '@/appflowy_app/stores/effects/database/cell/cell_bd_svc';
import { CellCache } from '@/appflowy_app/stores/effects/database/cell/cell_cache';
import { FieldController } from '@/appflowy_app/stores/effects/database/field/field_controller';
import { FieldType } from '@/services/backend';
import { BoardDateCell } from '../../board/BoardDateCell';
import { BoardUrlCell } from '../../board/BoardUrlCell';

import GridSingleSelectOptions from './GridSingleSelectOptions';
import GridTextCell from './GridTextCell';
import { GridCheckBox } from './GridCheckBox';

export const GridCell = ({
  cellIdentifier,
  cellCache,
  fieldController,
}: {
  cellIdentifier: CellIdentifier;
  cellCache: CellCache;
  fieldController: FieldController;
}) => {
  return (
    <>
      {cellIdentifier.fieldType === FieldType.MultiSelect || cellIdentifier.fieldType === FieldType.Checklist ? (
        <p> Select solutions</p>
      ) : cellIdentifier.fieldType === FieldType.SingleSelect ? (
        <GridSingleSelectOptions
          cellIdentifier={cellIdentifier}
          cellCache={cellCache}
          fieldController={fieldController}
        />
      ) : cellIdentifier.fieldType === FieldType.Checkbox ? (
        <GridCheckBox cellIdentifier={cellIdentifier} cellCache={cellCache} fieldController={fieldController} />
      ) : cellIdentifier.fieldType === FieldType.DateTime ? (
        <BoardDateCell
          cellIdentifier={cellIdentifier}
          cellCache={cellCache}
          fieldController={fieldController}
        ></BoardDateCell>
      ) : cellIdentifier.fieldType === FieldType.URL ? (
        <BoardUrlCell
          cellIdentifier={cellIdentifier}
          cellCache={cellCache}
          fieldController={fieldController}
        ></BoardUrlCell>
      ) : (
        <GridTextCell cellIdentifier={cellIdentifier} cellCache={cellCache} fieldController={fieldController} />
      )}
    </>
  );
};
