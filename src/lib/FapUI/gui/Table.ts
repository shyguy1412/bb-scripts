import { FapComponents, FapElement } from "@/lib/FapUI";


export function FapTable(data: readonly React.ReactNode[][], header: readonly [React.ReactNode[]?, React.ReactNode[]?] = [], transpose?: boolean) {
  const {
    Table, Th, Tr, Thead, Tbody, Td
  } = FapComponents;

  const table: FapElement<any>[] = [];

  const [columnHeader, rowHeader] = transpose ? header.concat(...Array(2 - header.length)).reverse() : header;

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

  console.log({transposedData});

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

export function AsciiTable(data: readonly string[][], transpose?: boolean) {
  const { Div } = FapComponents;

  const transposedData = transpose ?
    data.map((_, i, arr) => arr.map(row => row[i])).filter(arr => !arr.every(val => val == undefined)) :
    data;

  const colWidths = transposedData.reduce((prev, row) => prev.map((v, i) => row[i].length > v ? row[i].length : v), transposedData[0].map(v => v.length));
  const table = transposedData.reduce((prev, cur, i, { length }) => {

    const filler = colWidths.map(w => Array(w).fill(i && i != length - 1 ? '─' : '═').join('')).join(i && i != length - 1 ? '┼' : !i ? '╪' : '╧');

    let row = '';

    if (i == 0) {
      row += '╔' + filler.replaceAll('╪', '╤') + '╗' + '\n';
    }

    row += '║' + cur.map((v, i) => v.padEnd(colWidths[i], ' ')).join('│') + '║' + '\n';

    if (i == 0)
      row += '╠' + filler + '╣';
    else if (i != length - 1)
      row += '╟' + filler + '╢';
    else
      row += '╚' + filler + '╝';

    return prev + '\n' + row;
  }, '');


  return Div(table).Style({ lineHeight: 1 });
}