import axios from "axios";
import { popularGamesUrl, upComingGamesUrl, newGamesUrl } from '../api';

export const loadGames = () => async (dispatch) => {
    //fetch axios
    
    const popularData = await axios.get(popularGamesUrl());
    const upComingData = await axios.get(upComingGamesUrl());
    const newGamesData = await axios.get(newGamesUrl());

    dispatch({
        type: "FETCH_GAMES", 
        payload: {
            popular: popularData.data.results, 
            upcoming: upComingData.data.results, 
            newGames: newGamesData.data.results,
        }
    })
} 

