import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import {useEffect, useState} from 'react'
import Header from "../components/Header";
import SignUp from "../components/SignUp";
import Feed from "../components/Feed";
import Sidebar from "../components/Sidebar";
import RightSidebar from "../components/RightSidebar";
import { useWallet } from '@solana/wallet-adapter-react'

const style = {
    wrapper: `bg-[#18191a] min-h-screen duration-[0.5s]`,
    homeWrapper: `flex`,
    center: `flex-1`,
    main: `flex-1 flex justify-center`,
    signupContainer: `flex items-center justify-center w-screen h-[70vh]`,
}

const Home: NextPage = () => {
    const [registered, setRegistered] = useState(false)
    const [name, setName] = useState('')
    const [url, setUrl] = useState('')
    const [users, setUsers] = useState([]);

    useEffect(() => {
        ;(async () => {
            // @ts-ignore
            await requestUsersData()
        })()
    }, []);

    const wallet = useWallet()

    // @ts-ignore
    const requestUsersData = async activeAccount => {
        try {
            const response = await fetch(`/api/fetchUsers`)
            const data = await response.json()

            setUsers(data.data)
        } catch (error) {
            console.error(error)
        }
    }

  return (
   <div className={style.wrapper}>
       <Head>
           <title>Facebook Web3</title>
           <link rel="icon" href="/favicon.ico" />
       </Head>

        {/*@ts-ignore */}
        <Header name={name} url={url} />

        {registered ? (
            <div className={style.homeWrapper}>
                <Sidebar name={name} url={url} />
                <div className={style.main}>
                    {/* @ts-ignore */}
                    <Feed
                        connected={wallet.connected}
                        name={name} url={url} />
                </div>
                <RightSidebar
                    getUsers={requestUsersData}
                    users={users}
                    setUsers={setUsers}
                />
            </div>
        ) : (
            <div className={style.signupContainer}>
                <SignUp
                    setRegistered={setRegistered}
                    name={name}
                    setName={setName}
                    url={url}
                    setUrl={setUrl}
                />
            </div>
        )}
    </div>
  )
}

export default Home;
