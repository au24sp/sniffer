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
import { open } from '@tauri-apps/api/dialog';
import { join } from '@tauri-apps/api/path';


function App() {
  const [activeComponent, setActiveComponent] = useState("bargraph");
  const [currentPage, setCurrentPage] = useState("sniffer");
  const [isRunning, setIsRunning] = useState(false);
  const [tableNames, setTableNames] = useState([]);
  const [selectedTable, setSelectedTable] = useState("");
  const [tableData, setTableData] = useState([]);
  const [interfaces, setInterfaces] = useState([]);
  const [selectedInterface, setSelectedInterface] = useState("");
  const [analysisTable, setAnalysisTable] = useState("");
  const [packetTypesData, setPacketTypesData] = useState(null);

  useEffect(() => {
    if (currentPage === "table") {
      loadTableNames();
    }
    if (currentPage === "sniffer") {
      listInterfaces();
    }
    if (currentPage === "analysis") {
    }
  }, [currentPage]);

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
  const [ipStatsData, setIpStatsData] = useState(null);
  const [timestampData, setTimestampData] = useState(null);

  const generateJsonFiles = async () => {
    if (analysisTable) {
      await invoke("output_ip_stats_command", {
        tableName: analysisTable,
        outputFile: "ip_stats.json",
      });
      
      await invoke("output_packet_per_second_command", {
        tableName: analysisTable,
        outputFile: "timestamp_details.json",
      });

      await invoke("output_packet_types_command", {
        tableName: analysisTable,
        outputFile: "packet_types.json",
      });
  
      const ipStats = await invoke('read_ip_stats');
      const parsedIpStats = JSON.parse(ipStats);
      setIpStatsData(parsedIpStats);
  
      const timestampDetails = await invoke('read_timestamp_details');
      const parsedTimestampDetails = JSON.parse(timestampDetails);
      setTimestampData(parsedTimestampDetails);

      const packetTypes = await invoke('read_packet_types');
      const parsedPacketTypes = JSON.parse(packetTypes);
      setPacketTypesData(parsedPacketTypes);
  
      setActiveComponent(prev => prev);
    }
  };
  
  return (
    <div className="App min-h-screen bg-gray-100 flex flex-col">
      <nav className="bg-gray-800 p-4 text-white flex justify-between items-center">
        <div>
          <button
            onClick={() => setCurrentPage("sniffer")}
            className={`px-4 py-2 rounded ${
              currentPage === "sniffer"
                ? "bg-gray-700"
                : "bg-gray-600 hover:bg-gray-500"
            }`}
          >
            Packet Sniffer
          </button>
          <button
            onClick={() => setCurrentPage("table")}
            className={`px-4 py-2 rounded ml-4 ${
              currentPage === "table"
                ? "bg-gray-700"
                : "bg-gray-600 hover:bg-gray-500"
            }`}
          >
            Table View
          </button>
          <button
            onClick={() => setCurrentPage("analysis")}
            className={`px-4 py-2 rounded ml-4 ${
              currentPage === "table"
                ? "bg-gray-700"
                : "bg-gray-600 hover:bg-gray-500"
            }`}
          >
            Analysis
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
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500"
              disabled={isRunning}
            >
              Start Sniffer
            </button>
            <button
              onClick={stopSniffer}
              className="px-4 py-2 bg-red-600 text-blue-500 rounded hover:bg-red-500 ml-4"
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
      {currentPage === "analysis" && (
        <div>
          <h1 className="text-2xl font-bold mb-4">Analysis</h1>
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
              onClick={generateJsonFiles}
              className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-500 ml-4"
              disabled={!analysisTable}
            >
              Analyse
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
            <Button onClick={() => setActiveComponent("piechart")}
                disabled={activeComponent === "piechart"}>
                Show Piechart
              </Button>
          </div>
          <div className="scale-[0.90]">
          {activeComponent === "bargraph" && <Bargraph data={ipStatsData} />}
          </div>
          <div className="mt-[10%]">
          {activeComponent === "lineargraph" && <Lineargraph data={timestampData} />}
          <div className="mt-[10%]">
          {activeComponent === "piechart" && <Piechart data={packetTypesData} />}
        </div>
          </div>
        </div>
      )}
      </main>
    </div>
  );
}

export default App;
