import React, { PropsWithoutRef } from 'react';

type Props<T extends React.FunctionComponent> = {
  data: Parameters<T>[0][];
  li: T;
};

export function List<T extends React.FunctionComponent>({ data, li }: Props<T>) {
  return <>
    {data?.map(c => React.createElement(li, c))}
  </>;
}