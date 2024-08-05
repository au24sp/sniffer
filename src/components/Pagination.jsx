import { useEffect, useState } from "react";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { Button } from "./ui/button";

function Pagination({ tableData }) {
  const [currentPage, setCurrentPage] = useState(1);
  const [currentRecords, setCurrentRecords] = useState([]);
  const [noOfPages, setNoOfPages] = useState(0);

  useEffect(() => {
    setNoOfPages(Math.ceil(tableData.length / 15));
    const start = (currentPage - 1) * 15;
    const end = start + 15;
    setCurrentRecords(tableData.slice(start, end));
  }, [currentPage, tableData]);

  return (
    <>
      {currentRecords.length > 0 ? (
        <table className="min-w-full bg-white border border-gray-200">
          <thead>
            <tr>
              {Object.keys(currentRecords[0]).map((key) => (
                <th key={key} className="px-4 py-2 border-b">
                  {key}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {currentRecords.map((row, index) => (
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
      <div className="flex gap-8 items-center m-4 mx-auto w-full justify-center">
        <Button
          onClick={() => setCurrentPage(currentPage - 1)}
          disabled={currentPage === 1}
        >
          Previous
        </Button>
        <span>{currentPage}</span>
        <Button
          onClick={() => setCurrentPage(currentPage + 1)}
          disabled={currentPage === noOfPages}
        >
          Next
        </Button>
      </div>
    </>
  );
}

const PrettyTableRow = ({ value, index }) => {
  return (
    <>
      {typeof value === "object" ? (
        <td key={index} className="px-4 py-2 border-b">
          <ExpandDialog value={value} type="object" />
        </td>
      ) : typeof value === "string" ? (
        <td key={index} className="px-4 py-2 border-b">
          <ExpandDialog value={value} type="string" />
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
        {/* <ExpandDialog value={str} /> */}
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
        {/* <ExpandDialog value={JSON.stringify(object, null, 2)} /> */}
      </>
    );
  }
  return JSON.stringify(object, null, 2);
};

const ExpandDialog = ({ value, type }) => {
  return (
    <>
      <Dialog>
        <DialogTrigger asChilds>
          <Button className="rounded-sm bg-white text-black hover:bg-gray-300 hover:border-black">
            {type == "object"
              ? retractObject(value)
              : type == "string"
                ? retractString(value)
                : "Null type"}
          </Button>
        </DialogTrigger>
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

export default Pagination;
