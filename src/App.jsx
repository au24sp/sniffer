import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [currentPage, setCurrentPage] = useState('sniffer');
  const [isRunning, setIsRunning] = useState(false);
  const [tableNames, setTableNames] = useState([]);
  const [selectedTable, setSelectedTable] = useState('');
  const [tableData, setTableData] = useState([]);

  useEffect(() => {
    if (currentPage === 'table') {
      loadTableNames();
    }
  }, [currentPage]);

  const startSniffer = async () => {
    await invoke('start_packet_sniffer');
    setIsRunning(true);
  };

  const stopSniffer = async () => {
    await invoke('stop_packet_sniffer');
    setIsRunning(false);
  };

  const loadTableNames = async () => {
    try {
      const names = await invoke('get_table_names');
      setTableNames(names);
    } catch (error) {
      console.error('Error loading table names:', error);
    }
  };

  const fetchTableData = async () => {
    if (selectedTable) {
      try {
        const data = await invoke('get_table_data', { table: selectedTable });
        setTableData(data);
      } catch (error) {
        console.error('Error fetching table data:', error);
      }
    }
  };

  return (
    <div className="App min-h-screen bg-gray-100 flex flex-col">
      <nav className="bg-gray-800 p-4 text-white flex justify-between items-center">
        <div>
          <button
            onClick={() => setCurrentPage('sniffer')}
            className={`px-4 py-2 rounded ${
              currentPage === 'sniffer' ? 'bg-gray-700' : 'bg-gray-600 hover:bg-gray-500'
            }`}
          >
            Packet Sniffer
          </button>
          <button
            onClick={() => setCurrentPage('table')}
            className={`px-4 py-2 rounded ml-4 ${
              currentPage === 'table' ? 'bg-gray-700' : 'bg-gray-600 hover:bg-gray-500'
            }`}
          >
            Table View
          </button>
        </div>
      </nav>

      <main className="flex-grow p-4">
        {currentPage === 'sniffer' && (
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-4">Packet Sniffer</h1>
            <button
              onClick={startSniffer}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500"
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

        {currentPage === 'table' && (
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
                {tableNames.map((name) => (
                  <option key={name} value={name}>
                    {name}
                  </option>
                ))}
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
              {tableData.length > 0 && (
                <table className="min-w-full bg-white border border-gray-200">
                  <thead>
                    <tr>
                      {Object.keys(tableData[0]).map((key) => (
                        <th key={key} className="px-4 py-2 border-b">{key}</th>
                      ))}
                    </tr>
                  </thead>
                  <tbody>
                    {tableData.map((row, index) => (
                      <tr key={index}>
                        {Object.values(row).map((value, i) => (
                          <td key={i} className="px-4 py-2 border-b">
                            {typeof value === 'object' ? JSON.stringify(value) : value}
                          </td>
                        ))}
                      </tr>
                    ))}
                  </tbody>
                </table>
              )}
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
