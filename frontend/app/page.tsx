'use client';

import SearchBar from "@/components/common/SearchBar/SearchBar";
import RoomCard from "@/components/home/RoomCard";
import ArrowDownIcon from "@/components/common/icons/ArrowDownIcon";
import { GameInfo } from "@/types/game";
import { RoomInfo, RoomStatus } from "@/types/room";
import { randInt } from "@/utils/math";
import { Button, Checkbox, Typography } from "antd";
import Image from "next/image";

const { Text } = Typography;

const allGames: Pick<GameInfo, 'activeRooms' | 'img' | 'name'>[] = [
  {
    img: "https://brainium.com/wp-content/uploads/2021/11/sudoku-Mobile-hero-asset@2x.png",
    activeRooms: randInt(30, 100),
    name: "Sudoku"
  },
  {
    img: "https://funhtml5games.com/sokoban/sokobon_trans.png",
    activeRooms: randInt(30, 100),
    name: "Sokoban"
  }
]

const allRooms: RoomInfo[] = [
  {
    idByGame: 1,
    creator: "xion14",
    status: RoomStatus.Pending,
    playerCount: 2,
    maxPlayers: 4,
    gameInfo: {
      img: "https://reactjsexample.com/content/images/2020/04/A-Sudoku-web-app-in-React.png",
      name: "Sudoku",
      slug: "sudoku"
    },
    depositPrice: 10
  },
  {
    idByGame: 1,
    creator: "xion12",
    status: RoomStatus.Finished,
    playerCount: 3,
    maxPlayers: 4,
    gameInfo: {
      img: "https://static.tvtropes.org/pmwiki/pub/images/sokoban_6694.png",
      name: "Sokoban",
      slug: "sokoban"
    },
    depositPrice: 23
  },
  {
    idByGame: 2,
    creator: "xion14",
    status: RoomStatus.Playing,
    playerCount: 2,
    maxPlayers: 4,
    gameInfo: {
      img: "https://reactjsexample.com/content/images/2020/04/A-Sudoku-web-app-in-React.png",
      name: "Sudoku",
      slug: "sudoku"
    },
    depositPrice: 30.5
  },
]

export default function Home() {
  return (
    <section className="flex flex-col md:flex-row md:gap-6 pt-6 max-w-[1920px] mx-auto">
      <div className="bg-secondary/80 backdrop-blur-lg flex flex-col gap-8 w-full pb-2 md:w-[16rem] z-40 sticky top-[142px] md:top-[110px] md:overflow-auto md:h-[calc(100vh-180px)] scrollbar-none">
        <Button className="flex items-center gap-2 py-5" type="primary">
          {/* <SubmitNFTIcon /> */}
          <Text className="text-2xl font-extrabold">+</Text>
          <p className="text-sm font-bold max-w-lg ">Create new room</p>
        </Button>
        {/* <SearchBar className="w-full" /> */}
        <div className="flex flex-col gap-6 pb-10 font-medium">
          <div className="flex flex-col gap-4">
            <div className="flex items-center cursor-pointer">
              <Text strong className="flex-1">Games</Text>
              <ArrowDownIcon />
            </div>
            {
              allGames.map((game) => (
                <div className="flex items-center gap-2 text-muted cursor-pointer" key={game.name}>
                  <Checkbox></Checkbox>
                  <Image src={game.img} alt="" width={25} height={25} className="rounded-full" />
                  <Text className="flex-1 text-inherit uppercase">{game.name}</Text>
                  <Text className="text-inherit">{game.activeRooms}</Text>
                </div>
              ))
            }
          </div>
          <div className="flex flex-col gap-4">
            <div className="flex items-center cursor-pointer">
              <Text strong className="flex-1">Status</Text>
              <ArrowDownIcon />
            </div>
            {
              [RoomStatus.Playing, RoomStatus.Pending, RoomStatus.Finished].map((roomStatus) => (
                <div className="flex items-center gap-2 text-muted cursor-pointer" key={roomStatus}>
                  <Checkbox></Checkbox>
                  <Text className="flex-1 text-inherit uppercase">{roomStatus}</Text>
                  <Text className="text-inherit">{randInt(30, 100)}</Text>
                </div>
              ))
            }
          </div>
        </div>
      </div>
      <div className="flex-1 flex flex-col gap-8">
        <div className="flex flex-row justify-between items-start">
          <div>
            <h3 className="text-xl md:text-2xl font-bold">All rooms</h3>
            <span className="text-muted text-sm md:text-md">{allRooms.length}+ rooms</span>
          </div>
          <div className="flex justify-end">
            <div className="relative" data-headlessui-state="">
              <button
                className="flex gap-2 items-center transition-all p-2 rounded-md hover:bg-button-hover text-sm border-2 bg-transparent border-transparent"
              >
                <span className="font-bold">Sort by</span>
                <span>Most recent</span>
                <ArrowDownIcon />
              </button>
            </div>
          </div>
        </div>

        <SearchBar placeholder="Search by room ID, game, ..." className="w-full" />
        <div
          className="grid grid-cols-2 items-start lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-4 flex-1 mb-4">
          {
            allRooms?.map((room) => (
              <RoomCard key={room.idByGame + room.gameInfo.name} {...room} />
            ))
          }
        </div>
      </div>
    </section>
  );
}
