function isWhitespace(char: string) {
  return char && char.trim().length == 0;
}

function getLine(string: string, pos: number) {
  const section = string.slice(0, pos);
  return [...section].filter(c => c == '\n').length + 1;
}

function getColumn(string: string, pos: number) {
  const section = string.slice(0, pos);
  const line = section.split(/\n/).at(-1)!;
  return line.length + 1;
}

function getPosString(string: string, pos: number) {
  return `${getLine(string, pos)}:${getColumn(string, pos)}`;
}

function getWordEnd(string: string, pos: number, limit = string.length) {
  let i = 0;
  while (!isWhitespace(string[pos + i]) && i < limit) i++;
  return pos + i;
}

function isLineStart(string: string, pos: number) {
  return string[pos - 1] == undefined || string[pos - 1] == '\n';
}

function parseSection(source: string, pos: number) {
  let cursor = pos;
  if (!isLineStart(source, cursor)) {
    throw new SyntaxError(
      `Expected start of section at ${getPosString(source, cursor)} got whitespace. A section identifier should not have any whitespace!`
    );
  }

  if (source[cursor] != '[') {
    throw new SyntaxError(
      `Expected start of section at ${getPosString(source, cursor)} got '${source.slice(cursor, getWordEnd(source, cursor, 10))}'`
    );
  }

  cursor++;

  const sectionStart = cursor;

  while (source[cursor] && source[cursor] != ']' && source[cursor] != '\n') cursor++;

  const section = source.slice(sectionStart, cursor);


  if (source[cursor] == undefined) {
    throw new SyntaxError(
      `Unexpected end of file: No closing bracket for section identifier '${section}'`
    );
  }

  if (source[cursor] == '\n') {
    throw new SyntaxError(
      `Unexpected end of line: No closing bracket for section identifier '${section}' at ${getPosString(source, cursor)}`
    );
  }

  if (!/^[A-Z]/.test(section)) {
    throw new SyntaxError(
      `Invalid section name '${section}' at ${getPosString(source, cursor)}: A section name should always start with an uppercase letter`
    );
  }

  if (!/^(X-)|[A-Za-z]*$/.test(section)) {
    throw new SyntaxError(
      `Invalid section name '${section}' at ${getPosString(source, cursor)}: A section name can only consist of letters`
    );
  }

  cursor++;
  return { section, pos: cursor };
}

function parseDirective(source: string, pos: number) {
  let cursor = pos;
  if (!isLineStart(source, cursor)) {
    throw new SyntaxError(
      `Unexpected whitespace at ${getPosString(source, cursor)}: Expected directive`
    );
  }

  const directiveStart = cursor;
  while (source[cursor] && source[cursor] != '=') cursor++;
  const directive = source.slice(directiveStart, cursor);

  if (source[cursor] == undefined) {
    throw new SyntaxError(
      `Unexpected end of file: Expected '=' after directive '${directive}'`
    );
  }

  if (!/^[A-Z]/.test(directive)) {
    throw new SyntaxError(
      `Invalid directive '${directive}' at ${getPosString(source, cursor)}: A directive should always start with an uppercase letter`
    );
  }

  if (!/^[A-Za-z]*$/.test(directive)) {
    throw new SyntaxError(
      `Invalid directive '${directive}' at ${getPosString(source, cursor)}: A directive can only consist of letters`
    );
  }

  cursor++;

  const valueStart = cursor;
  while (source[cursor] && source[cursor] != '\n') cursor++;

  const value = source.slice(valueStart, cursor);

  cursor++;

  return { directive, value, pos: cursor };
}

export function parseService(raw: string) {

  const source = raw.replace(/\r/g, '');
  let cursor = 0;

  const service: Record<string, Record<string, string>> = {};

  while (isWhitespace(source[cursor])) cursor++;

  while (cursor < source.length) {
    const { section, pos } = parseSection(source, cursor);
    cursor = pos;

    service[section] = {};

    while (isWhitespace(source[cursor])) cursor++;
    while (source[cursor] && source[cursor] != '[') {

      const { directive, value, pos } = parseDirective(source, cursor);
      cursor = pos;
      service[section][directive] = value;

      while (isWhitespace(source[cursor])) cursor++;

    }
  }


  return service;
}