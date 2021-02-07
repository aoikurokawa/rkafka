import React from "react";
//global style
import GlobalStyle from './component/GlobalStyle';

import AboutUs from "./pages/AboutUs";

function App() {
  return (
    <div className="App">
      <GlobalStyle />
      <AboutUs />
    </div>
  );
}

export default App;
