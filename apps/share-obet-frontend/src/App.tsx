import {useEffect} from 'react'
import './App.css'
// import WebApp from "@twa-dev/sdk";
// import type { WebApp  } from "@tma.js/types";

// export const tg = window.Telegram?.WebApp;
//
// https://www.npmjs.com/package/@ton/ton?utm_source=chatgpt.com
// https://www.npmjs.com/package/@tonconnect/ui-react?utm_source=chatgpt.com

import { postEvent, mainButton } from "@tma.js/sdk";
import { useLaunchParams } from "@tma.js/sdk-react";



function App() {
  // const [count, setCount] = useState(0)
    const { tgWebAppData } = useLaunchParams();

    const user = tgWebAppData?.user

    useEffect(() => {
        console.log(user);
    }, [])

    useEffect(() => {
        postEvent("web_app_expand");
    }, [])

    useEffect(() => {
        mainButton.setParams({
            text: "Create pledge",
            isVisible: true,
        });

        const off = mainButton.onClick(() => {
            console.log("click");
        });

        return () => {
            off();
            mainButton.hide();
        };
    }, []);



    return (
    <>
        PRIVET!!!!!!!!!!!!!!
        <p>{user?.first_name}</p>
        <p>@{user?.username}</p>
        <p>{user?.id}</p>
    </>
  )
}

export default App
