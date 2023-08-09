import Image from 'next/image'
import Game from './game';

export default async function Home() {



  return (
    <main className="">
        <div className="bg-slate-700 text-slate-50">
            <Game />
        </div>
    </main>
  )
}
