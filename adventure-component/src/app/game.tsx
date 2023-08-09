"use client"

import useSWR from 'swr'
import {useState} from 'react'

async function getCommands(url: string) {
    const response = await fetch(url)
    if (!response.ok) {
        throw new Error('Network response was not ok.')
    }
    const commands = await response.json()
    return commands
} 

const base_url = "http://localhost:3000/api/adventure"
export default function Game() {
    const [commands, setCommands] = useState([])
    const { data, error, isLoading } = useSWR(`${base_url}?commands=${JSON.stringify({
        commands: []
    })}`, (url) => getCommands(url)
    )

    if (error) return <div>failed to load</div>
    if (isLoading) return <div>loading...</div>
    
  return (
    <div>
        {/* adventure text */}
        <div>
            {data.map((ins: string) =>(
                <p>{ins}</p>
            ))}
        </div>
        {/* send button */}
        <div className="flex items-center border-t border-black py-2">
            <input className="appearance-none bg-transparent border-none w-full mr-3 py-1 px-2 leading-tight text-slate-50 focus:outline-none" type="text" placeholder="Command" />
            <button className="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-slate-700 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded" type="button">
            Send
            </button>
        </div>
            
    </div>
  )
}
