import {useEffect} from 'react'
import './App.css'
// https://www.npmjs.com/package/@ton/ton?utm_source=chatgpt.com
// https://www.npmjs.com/package/@tonconnect/ui-react?utm_source=chatgpt.com

import {
    mainButton,
    miniApp } from "@tma.js/sdk";
import { useLaunchParams } from "@tma.js/sdk-react";



function App() {
    const { tgWebAppData } = useLaunchParams();

    const user = tgWebAppData?.user

    useEffect(() => {
        console.log(user);
    }, [user])

    useEffect(() => {
        miniApp.mount();

        miniApp.ready();

        // return () => {
        //     miniApp.unmount();
        // };
    }, []);

    useEffect(() => {
        mainButton.mount();

        mainButton.setParams({
            text: "Create pledge",
            isVisible: true,
        });

        const off = mainButton.onClick(() => {
            console.log("Create pledge click");
        });

        return () => {
            off();

            // mainButton.unmount();
        };
    }, []);



    return (
    <>
        {!user && (
            <p>
                Open inside Telegram
            </p>
        )}

        PRIVET!!!!!!!!!!!!!!
        {user && (
            <>
                {user.photo_url && (
                    <img
                        src={user.photo_url}
                        alt={user.first_name}
                        width={96}
                        height={96}
                        style={{
                            borderRadius: "50%",
                        }}
                    />
                )}

                <p>
                    {user.first_name}{" "}
                    {user.last_name}
                </p>

                <p>
                    @{user.username}
                </p>

                <p>
                    ID: {user.id}
                </p>

                <p>
                    Premium:{" "}
                    {user.is_premium
                        ? "YES"
                        : "NO"}
                </p>

                <p>
                    Language:{" "}
                    {user.language_code}
                </p>
            </>
        )}
    </>
  )
}

export default App
