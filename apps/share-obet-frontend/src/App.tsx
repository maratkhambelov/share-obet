import {useEffect} from 'react'
import './App.css'
// import WebApp from "@twa-dev/sdk";

function App() {
  // const [count, setCount] = useState(0)

    useEffect(() => {
        // WebApp.ready();
    }, [])

  return (
    <>
        PRIVET!!!!!!!!!!!!!!
        {/*User: {WebApp.initDataUnsafe.user?.first_name}*/}
    </>
  )
}

export default App
