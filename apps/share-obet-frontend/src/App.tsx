import {useEffect} from 'react'
import './App.css'
// import WebApp from "@twa-dev/sdk";
// import type { WebApp  } from "@tma.js/types";

// export const tg = window.Telegram?.WebApp;
//

import { init } from "@tma.js/sdk";
import { postEvent, mainButton } from "@tma.js/sdk";
import { useLaunchParams } from "@tma.js/sdk-react";


init()

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
