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
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

function App() {
  const [currentPage, setCurrentPage] = useState("sniffer");
  const [isRunning, setIsRunning] = useState(false);
  const [tableNames, setTableNames] = useState([]);
  const [selectedTable, setSelectedTable] = useState("");
  const [tableData, setTableData] = useState([]);
  const [interfaces, setInterfaces] = useState([]);
  const [selectedInterface, setSelectedInterface] = useState("");

  useEffect(() => {
    if (currentPage === "table") {
      loadTableNames();
    }
    if (currentPage === "sniffer") {
      listInterfaces();
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

  return (
    <>
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
          </div>
        </nav>

        <main className="flex-grow p-4">
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
                    setTableData([]); // Clear previous data
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
                {tableData.length > 0 ? (
                  <table className="min-w-full bg-white border border-gray-200">
                    <thead>
                      <tr>
                        {Object.keys(tableData[0]).map((key) => (
                          <th key={key} className="px-4 py-2 border-b">
                            {key}
                          </th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {tableData.map((row, index) => (
                        <tr key={index}>
                          {Object.values(row).map((value, i) => (
                            <PrettyTableRow key={i} value={value} index={i} />
                          ))}
                        </tr>
                      ))}
                    </tbody>
                  </table>
                ) : (
                  <p>No data available</p>
                )}
              </div>
            </div>
          )}
        </main>
      </div>
    </>
  );
}

export default App;

const PrettyTableRow = ({ value, index }) => {
  return (
    <>
      {typeof value === "object" ? (
        <td key={index} className="px-4 py-2 border-b">
          {retractObject(value)}
        </td>
      ) : typeof value === "string" ? (
        <td key={index} className="px-4 py-2 border-b">
          {retractString(value)}
        </td>
      ) : (
        value
      )}
    </>
  );
};

const retractString = (str) => {
  if (str.length > 15) {
    return (
      <>
        {`${str.substring(0, 10)}...${str.substring(str.length - 5)}`}
        <ExpandDialog value={str} />
      </>
    );
  }
  return str;
};

const retractObject = (object) => {
  const keys = Object.keys(object);
  if (keys.length > 7) {
    const firstFiveKeys = keys.slice(0, 5);
    const lastTwoKeys = keys.slice(-2);
    const newObj = firstFiveKeys.reduce(
      (acc, key) => ({ ...acc, [key]: object[key] }),
      {}
    );
    newObj["..."] = "...";
    lastTwoKeys.forEach((key) => (newObj[key] = object[key]));
    return (
      <>
        {JSON.stringify(newObj, null, 2)}
        <ExpandDialog value={JSON.stringify(object, null, 2)} />
      </>
    );
  }
  return JSON.stringify(object, null, 2);
};

const ExpandDialog = ({ value }) => {
  return (
    <>
      <Dialog>
        <DialogTrigger>Open</DialogTrigger>
        <DialogContent className="h-64 flex justify-center items-center overflow-auto">
          <pre
            style={{
              whiteSpace: "pre-wrap",
              wordWrap: "break-word",
              overflowX: "auto",
              maxHeight: "100%",
            }}
          >
            {value}
          </pre>
        </DialogContent>
      </Dialog>
    </>
  );
};
