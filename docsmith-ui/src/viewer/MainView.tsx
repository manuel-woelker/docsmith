import {Navbar} from "./Navbar";
import {ContentView} from "./ContentView";
import {OutlineView} from "./OutlineView";

export const MainView = () => {
  return <div style={{"background-color": "red", height: "100vh", width: "100vw"}}>
    <div style={{"background-color": "blue", height: "20px", width: "100%", position: "absolute"}}>
      <Navbar />
    </div>
    <div style={{"background-color": "green", "overflow-y": "scroll", height: "calc(100vh - 20px)", width: "200px", position: "absolute", "top": "20px"}}>
      <OutlineView />
    </div>
    <div style={{"background-color": "yellow", "overflow-y": "scroll", height: "calc(100vh - 20px)", width: "calc(100vw - 200px)", position: "absolute", "top": "20px", left: "200px"}}>
      <ContentView />
    </div>
  </div>;


};
