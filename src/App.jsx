import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./index.css";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "./components/ui/button";
import Pagination from "./components/Pagination";
import { Bargraph } from "./components/Bargraph";
import { Piechart } from "./components/Piechart";
import { Lineargraph } from "./components/lineargraph";
import { OllamaDataDisplay } from "./components/Ollamadisplay";
import { miyagi } from "ldrs";
import { open } from "@tauri-apps/api/dialog";
import { join } from "@tauri-apps/api/path";

function App() {
  const [activeComponent, setActiveComponent] = useState("bargraph");
  const [currentPage, setCurrentPage] = useState("sniffer");
  const [isRunning, setIsRunning] = useState(false);
  const [tableNames, setTableNames] = useState([]);
  const [selectedTable, setSelectedTable] = useState("NONE");
  const [tableData, setTableData] = useState([]);
  const [interfaces, setInterfaces] = useState([]);
  const [selectedInterface, setSelectedInterface] = useState("");
  const [analysisTable, setAnalysisTable] = useState("");
  const [ipStatsData, setIpStatsData] = useState([]);
  const [timestampData, setTimestampData] = useState([]);
  const [packetTypesData, setPacketTypesData] = useState([]);
  const [ollamaData, setOllamaData] = useState([]);

  const [isLoading, setIsLoading] = useState(false);
  const [protocol, setProtocol] = useState("");
  const [source_ip, setSourceIP] = useState("");
  const [destination_ip, setDestinationIP] = useState("");
  const [src_ip_list, setSrcIpList] = useState([]);
  const [dest_ip_list, setDestIpList] = useState([]);
  const [protocol_list, setProtocolList] = useState([]);

  useEffect(() => {
    if (currentPage === "table") {
      loadTableNames();
    }
    if (currentPage === "sniffer") {
      listInterfaces();
    }
    if (currentPage === "visualization") {
      loadTableNames();
    }
    if (currentPage === "analysis") {
      loadTableNames();
      loadAnalysis();
    }
  }, [currentPage, analysisTable]);

  const startSniffer = async () => {
    if (selectedInterface) {
      try {
        await invoke("start_packet_sniffer", { interface: selectedInterface });
        setIsRunning(true);
      } catch (error) {
        console.error("Error starting sniffer:", error);
      }
    } else {
      alert("Please select an interface before starting the sniffer.");
    }
  };

  const listInterfaces = async () => {
    try {
      const res = await invoke("list_interfacce");
      setInterfaces(res);
    } catch (error) {
      setInterfaces([]);
      console.error("Error listing interfaces:", error);
    }
  };

  const stopSniffer = async () => {
    try {
      await invoke("stop_packet_sniffer");
      setIsRunning(false);
    } catch (error) {
      console.error("Error stopping sniffer:", error);
    }
  };

  const loadTableNames = async () => {
    try {
      const names = await invoke("list_names");
      setTableNames(names);
    } catch (error) {
      console.error("Error loading table names:", error);
    }
  };

  const loadAnalysis = async () => {
    try {
      const ips = await invoke("list_src_ips", { table: analysisTable });
      setSrcIpList(ips);
      const ip = await invoke("list_dst_ips", { table: analysisTable });
      setDestIpList(ip);
      const prot = await invoke("list_protocol", { table: analysisTable });
      setProtocolList(prot);
    } catch (err) {
      console.log(err);
    }
  };

  const fetchTableData = async () => {
    if (selectedTable) {
      try {
        const data = await invoke("get_table_data", { table: selectedTable });
        setTableData(data);
      } catch (error) {
        console.error("Error fetching table data:", error);
      }
    }
  };

  // const fetchOllamaData = async () => {
  //   setIsLoading(true); // Start loading
  //   if (selectedTable) {
  //     try {
  //       const data = await invoke("handle_ollama", { table: analysisTable });
  //       setOllamaData(data);
  //       setIsLoading(false); // Stop loading once data is fetched
  //     } catch (error) {
  //       console.error("Error fetching table data:", error);
  //     }
  //   }
  // };

  const fetchOllamaDatapackets = async () => {
    setIsLoading(true); // Start loading
    print("fetching data before if");
    if (selectedTable) {
      try {
        print("fetching data");
        const data = await invoke("ollama_frontend", {
          table: analysisTable,
          protocol: protocol || "",
          sourceIp: source_ip,
          destinationIp: destination_ip,
        });
        setOllamaData(data);
        setIsLoading(false); // Stop loading once data is fetched
      } catch (error) {
        console.error("Error fetching table data:", error);
      }
    }
  };

  const generateVisualizationData = async () => {
    if (analysisTable) {
      try {
        const ipStats = await invoke("get_ip_stats", {
          tableName: analysisTable,
        });
        setIpStatsData(ipStats);

        const timestampDetails = await invoke("get_packet_per_second", {
          tableName: analysisTable,
        });
        setTimestampData(timestampDetails);

        const packetTypes = await invoke("get_packet_types", {
          tableName: analysisTable,
        });
        setPacketTypesData(packetTypes);

        setActiveComponent((prev) => prev);
      } catch (error) {
        console.error("Error fetching visualization data:", error);
      }
    }
  };

  return (
    <div className="App min-h-screen bg-gray-100 flex flex-col">
      <nav className="bg-yellow-950 p-4 text-white flex justify-between items-center">
        <div>
          <button
            onClick={() => setCurrentPage("sniffer")}
            className={`px-4 py-2 rounded ${
              currentPage === "sniffer"
                ? "bg-yellow-800"
                : "bg-yellow-900 hover:bg-yellow-700"
            }`}
          >
            Packet Sniffer
          </button>
          <button
            onClick={() => setCurrentPage("table")}
            className={`px-4 py-2 rounded ml-4 ${
              currentPage === "table"
                ? "bg-yellow-800"
                : "bg-yellow-900 hover:bg-yellow-700"
            }`}
          >
            Table View
          </button>
          <button
            onClick={() => setCurrentPage("visualization")}
            className={`px-4 py-2 rounded ml-4 ${
              currentPage === "visualization"
                ? "bg-yellow-800"
                : "bg-yellow-900 hover:bg-yellow-700"
            }`}
          >
            Visualization
          </button>
          <button
            onClick={() => setCurrentPage("analysis")}
            className={`px-4 py-2 rounded ml-4 ${
              currentPage === "analysis"
                ? "bg-yellow-800"
                : "bg-yellow-900 hover:bg-yellow-700"
            }`}
          >
            AI Analysis
          </button>
        </div>
      </nav>

      <main className="flex-grow p-4 overflow-auto">
        {currentPage === "sniffer" && (
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-4">Packet Sniffer</h1>

            <div className="mb-4">
              <select
                onChange={(e) => setSelectedInterface(e.target.value)}
                value={selectedInterface}
                className="px-4 py-2 border rounded"
              >
                <option value="">Select Interface</option>
                {interfaces.length > 0 ? (
                  interfaces.map((iface) => (
                    <option key={iface.name} value={iface.name}>
                      {iface.name}
                    </option>
                  ))
                ) : (
                  <option disabled>No interfaces available</option>
                )}
              </select>
            </div>

            <button
              onClick={startSniffer}
              className="px-4 py-2 bg-blue-800 text-white rounded hover:bg-blue-500"
              disabled={isRunning}
            >
              Start Sniffer
            </button>
            <button
              onClick={stopSniffer}
              className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-500 ml-4"
              disabled={!isRunning}
            >
              Stop Sniffer
            </button>
          </div>
        )}

        {currentPage === "table" && (
          <div>
            <h1 className="text-2xl font-bold mb-4">Table View</h1>
            <div className="mb-4">
              <select
                onChange={(e) => {
                  setSelectedTable(e.target.value);
                  setTableData([]);
                }}
                value={selectedTable}
                className="px-4 py-2 border rounded"
              >
                <option value="">Select Table</option>
                {tableNames.length > 0 ? (
                  tableNames.map((name) => (
                    <option key={name} value={name}>
                      {name}
                    </option>
                  ))
                ) : (
                  <option disabled>No tables available</option>
                )}
              </select>
              <button
                onClick={fetchTableData}
                className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-500 ml-4"
                disabled={!selectedTable}
              >
                Load Table Data
              </button>
            </div>
            <div className="mt-4">
              {tableData && <Pagination tableData={tableData} />}
            </div>
          </div>
        )}
        {currentPage === "visualization" && (
          <div>
            <h1 className="text-2xl font-bold mb-4">Visualization</h1>
            <div className="mb-4">
              <select
                onChange={(e) => setAnalysisTable(e.target.value)}
                value={analysisTable}
                className="px-4 py-2 border rounded"
              >
                <option value="">Select Table</option>
                {tableNames.map((name) => (
                  <option key={name} value={name}>
                    {name}
                  </option>
                ))}
              </select>
              <button
                onClick={generateVisualizationData}
                className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-500 ml-4"
                disabled={!analysisTable}
              >
                Visualize
              </button>
            </div>
            <div className="flex gap-7">
              <Button
                onClick={() => setActiveComponent("bargraph")}
                disabled={activeComponent === "bargraph"}
              >
                Show Bargraph
              </Button>
              <Button
                onClick={() => setActiveComponent("lineargraph")}
                disabled={activeComponent === "lineargraph"}
              >
                Show Lineargraph
              </Button>
              <Button
                onClick={() => setActiveComponent("piechart")}
                disabled={activeComponent === "piechart"}
              >
                Show Piechart
              </Button>
            </div>
            <div className="scale-[0.90]">
              {activeComponent === "bargraph" && (
                <Bargraph data={ipStatsData} />
              )}
            </div>
            <div className="mt-[10%]">
              {activeComponent === "lineargraph" && (
                <Lineargraph data={timestampData} />
              )}
              <div className="mt-[10%]">
                {console.log(ipStatsData)}
                {activeComponent === "piechart" && (
                  <Piechart data={packetTypesData} />
                )}
              </div>
            </div>
          </div>
        )}
        {currentPage === "analysis" && (
          <div className="flex flex-col items-center justify-center ">
            <h1 className="text-3xl font-bold mb-6">AI Analysis</h1>
            <div className="w-full max-w-md">
              <div className="mb-4">
                <select
                  onChange={(e) => setAnalysisTable(e.target.value)}
                  value={analysisTable}
                  className="w-full px-4 py-2 border rounded"
                >
                  <option value="">Select Table</option>
                  {tableNames.map((name) => (
                    <option key={name} value={name}>
                      {name}
                    </option>
                  ))}
                </select>
              </div>

              <div className="grid gap-4 mb-4">
                <select
                  value={protocol}
                  onChange={(e) => setProtocol(e.target.value)}
                  className="w-full px-4 py-2 border rounded"
                >
                  <option value="">Select Protocol</option>
                  {protocol_list.length > 0 ? (
                    protocol_list.map((name) => (
                      <option key={name} value={name}>
                        {name}
                      </option>
                    ))
                  ) : (
                    <option disabled>No tables available</option>
                  )}
                </select>

                <select
                  value={source_ip}
                  onChange={(e) => setSourceIP(e.target.value)}
                  className="w-full px-4 py-2 border rounded"
                >
                  <option value="">Select Source IP</option>
                  {src_ip_list.length > 0 ? (
                    src_ip_list.map((name) => (
                      <option key={name} value={name}>
                        {name}
                      </option>
                    ))
                  ) : (
                    <option disabled>No tables available</option>
                  )}
                </select>

                <select
                  value={destination_ip}
                  onChange={(e) => setDestinationIP(e.target.value)}
                  className="w-full px-4 py-2 border rounded"
                >
                  <option value="">Select Destination IP</option>
                  {dest_ip_list.length > 0 ? (
                    dest_ip_list.map((name) => (
                      <option key={name} value={name}>
                        {name}
                      </option>
                    ))
                  ) : (
                    <option disabled>No tables available</option>
                  )}
                </select>
              </div>

              <button
                onClick={fetchOllamaDatapackets}
                className="w-full px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 transition duration-300 ease-in-out transform hover:-translate-y-1"
                disabled={!analysisTable || isLoading}
              >
                Analyse
              </button>
            </div>
            <div className="mt-8 ">
              {isLoading ? (
                <div className="flex items-center justify-center ">
                  <div className="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-blue-500"></div>
                </div>
              ) : (
                <OllamaDataDisplay ollamaData={ollamaData} />
              )}
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
