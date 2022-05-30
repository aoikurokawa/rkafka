import { motion, AnimatePresence, AnimateSharedLayout } from "framer-motion";
import React, { useEffect } from "react";
//redux
import { useDispatch, useSelector } from "react-redux";
import { useLocation } from "react-router-dom";
import styled from "styled-components";
import { loadGames } from "../actions/gamesAction";
//components
import Game from "../components/Game";
import GameDetail from "../components/GameDetail";
//animation
import { fadeIn, popup } from "../animation";
//antd
import "antd/dist/antd.css";
import { Spin } from "antd";

const Home = () => {
  //get the current location
  const location = useLocation();
  const pathId = location.pathname.split("/")[2];
  //fetch the data
  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(loadGames());
  }, [dispatch]);
  //get that data back
  const { popular, newGames, upcoming, searched, isloaded } = useSelector(
    (state) => state.games
  );

  const { getDetail } = useSelector((state) => state.detail);

  return (
    <div>
      <GameList variants={fadeIn} initial="hidden" animate="show">
        <AnimateSharedLayout type="crossfade">
          <AnimatePresence>
            {pathId && <GameDetail pathId={pathId} getDetail={getDetail} />}
          </AnimatePresence>
          {searched.length ? (
            <div className="searched">
              <h2>Searched Games</h2>
              <Games>
                {searched.map((game) => (
                  <Game
                    name={game.name}
                    released={game.released}
                    id={game.id}
                    image={game.background_image}
                    key={game.id}
                  />
                ))}
              </Games>
            </div>
          ) : (
            ""
          )}
          <h2>Upcoming Games</h2>
          {isloaded ? (
            <StyledSpin>
              <Spin />
            </StyledSpin>
          ) : (
            <Games>
              {upcoming.map((game) => (
                <Game
                  name={game.name}
                  released={game.released}
                  id={game.id}
                  image={game.background_image}
                  key={game.id}
                />
              ))}
            </Games>
          )}

          <h2>Popular Games</h2>
          {isloaded ? (
            <StyledSpin>
              <Spin />
            </StyledSpin>
          ) : (
            <Games>
              {popular.map((game) => (
                <Game
                  name={game.name}
                  released={game.released}
                  id={game.id}
                  image={game.background_image}
                  key={game.id}
                />
              ))}
            </Games>
          )}

          <h2>New Games</h2>
          {isloaded ? (
            <StyledSpin>
              <Spin />
            </StyledSpin>
          ) : (
            <Games>
              {newGames.map((game) => (
                <Game
                  name={game.name}
                  released={game.released}
                  id={game.id}
                  image={game.background_image}
                  key={game.id}
                />
              ))}
            </Games>
          )}
        </AnimateSharedLayout>
      </GameList>
    </div>
  );
};

const GameList = styled(motion.div)`
  padding: 0rem 5rem;
  h2 {
    padding: 5rem 0rem;
  }
`;

const Games = styled(motion.div)`
  min-height: 80vh;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
  grid-column-gap: 3rem;
  grid-row-gap: 5rem;
`;

const StyledSpin = styled.div`
  margin: 20px 0;
  margin-bottom: 20px;
  padding: 30px 50px;
  text-align: center;
  border-radius: 4px;
`;

export default Home;
