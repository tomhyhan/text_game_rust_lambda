import Image from 'next/image'

export default async function Home() {
    let data = await fetch('http://localhost:3000/api/adventure')
    const instructions = await data.json();

  return (
    <main className="">
        {instructions.map((ins: string) => 
        <p>{ins}</p>)}
    </main>
  )
}
