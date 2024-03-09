import React from 'react';

type Props<T extends {}> = {
  data: T[];
  li: React.FunctionComponent<T>;
};

export function List<T extends {}>({ data, li }: Props<T>) {
  return <>
    {data?.map(c => React.createElement(li, { ...c, key: JSON.stringify(c) }))}
  </>;
}