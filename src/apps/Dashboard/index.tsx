import { NetscriptContext } from '@/lib/WindowApp';
import style from './Dashboard.module.css';
import React, { useContext, useEffect, useState } from 'react';

import { connect_to_nukeall } from '@/home/bin/nukeall';

export function Dashboard() {
    const ns = useContext(NetscriptContext);

    const [notNuked, setNotNuked] = useState([] as string[]);

    // useEffect(() => {
    //     const [write, read] = connect_to_nukeall(ns);
    //     let interval = setInterval(() => {
    //         write('get');
    //         read().then((r) => {
    //             if (r.isError()) {
    //                 return;
    //             }
    //             setNotNuked(r.value.data[0]);
    //         });
    //     }, 1000);

    //     return () => clearInterval(interval);
    // }, [...notNuked]);

    return <div className={style.dashboard}>
        <ul>{notNuked.map((s) => <li>{s}</li>)}</ul>
    </div>;
}
