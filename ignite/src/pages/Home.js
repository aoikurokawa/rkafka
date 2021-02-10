import { motion } from "framer-motion";
import React, { useEffect } from "react";
//redux
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";
import { loadGames } from '../actions/gamesAction';
//components
import Game from "../components/Game";

const Home = () => {
    //fetch the data
    const dispatch = useDispatch();
    useEffect(() => {
        dispatch(loadGames());
    }, [dispatch]);
    //get that data back
    const { popular, newGames, upcoming } = useSelector((state) => state.games);
    console.log(popular)

    return (
        <div>
            <GameList>
                <h2>Upcoming Games</h2>
                <Games>
                    {upcoming.map((game) => (
                        <Game name={game.name} released={game.released} id={game.id} image={game.background_image} key={game.id} />
                    ))}
                </Games>
            </GameList>
        </div>
    )
}

const GameList = styled(motion.div)`
    padding: 0rem 5rem;
    h2 {
        padding: 5rem 0rem;
    }
`

const Games = styled(motion.div)`
    min-height: 80vh;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
    grid-column-gap: 3rem;
    grid-row-gap: 5rem;

`

export default Home;
