import { FapComponents, FapElement } from "@/FapUI";


export function FapTable(data: React.ReactNode[][], header: [React.ReactNode[]?, React.ReactNode[]?] = [], transpose?: boolean) {
  const {
    Table, Th, Tr, Thead, Tbody, Td
  } = FapComponents;
  
  const table: FapElement<any>[] = [];

  const [columnHeader, rowHeader] = transpose ? header.toReversed() : header;

  if (columnHeader) table.push(
    Thead(
      Tr(
        [transpose ? (rowHeader ?? [])[0] : [], columnHeader].flat().map(col => Th(col))
      )
    )
  );

  const transposedData = transpose ?
    data.map((_, i, arr) => arr.map(row => row[i])).filter(arr => !arr.every(val => val == undefined)) :
    data;
  const dataWithHeader = transposedData.map((row, i) => rowHeader ? [rowHeader[transpose ? i + 1 : i], ...row] : row);

  table.push(
    Tbody(
      dataWithHeader.map(row => Tr([
        row.map((val, i) => (i == 0 && rowHeader ? Th : Td)(
          val
        ))
      ]))
    )
  );

  return Table(table);
}