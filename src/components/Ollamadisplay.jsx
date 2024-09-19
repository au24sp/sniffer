import React from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import Markdown from 'react-markdown'

export function OllamaDataDisplay({ ollamaData }) {
  // Check if ollamaData is an object and not null

// Assuming ollamaData is available in the scope
if (!ollamaData || typeof ollamaData.response !== 'string') {
    return <div>No valid response data available</div>;
  }
  
  let responseText = '';
  try {
    const responseObject = JSON.parse(ollamaData.response);
    responseText = responseObject.response;
  } catch (error) {
    console.error('Error parsing JSON:', error);
    return <div>Error parsing response data</div>;
  }
  
  // Continue with your component rendering using responseText
  return (
    <div className="m-4">
      <Card className="mb-4">
        <CardHeader>
          <CardTitle>Response Data</CardTitle>
        </CardHeader>
        <CardContent>
          <Markdown>{responseText}</Markdown>
        </CardContent>
      </Card>
    </div>
  );
  // const { model, created_at, done, done_reason } = response;

//   return (
//     <div className="m-4">
//       <Card className="mb-4">
//         <CardHeader>
//           <CardTitle>Response Data</CardTitle>
//         </CardHeader>
//         <CardContent>
//           {/* <p>
//             <strong>Model:</strong> {model}
//           </p>
//           <p>
//             <strong>Created At:</strong> {created_at}
//           </p>
         
//           <p>
//             <strong>Done:</strong> {done ? "Yes" : "No"}
//           </p>
//           <p>
//             <strong>Done Reason:</strong> {done_reason}
//           </p> */}
//           <p>
//             <strong>Response:</strong> {responseText}
//           </p>
//         </CardContent>
//       </Card>
//     </div>
//   );
}
