import * as React from "react";
import { CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";

const chartData = [
  { timeStamp: "11:20:19", traffic: 222 },
  { timeStamp: "11:20:20", traffic: 97 },
  { timeStamp: "11:20:21", traffic: 167 },
  { timeStamp: "11:20:22", traffic: 245 },
  { timeStamp: "11:20:22", traffic: 409 },
  { timeStamp: "11:20:22", traffic: 59 },
  { timeStamp: "11:20:22", traffic: 261 },
  { timeStamp: "11:20:23", traffic: 301 },
  { timeStamp: "11:20:24", traffic: 242 },
  { timeStamp: "11:20:25", traffic: 373 },
];

const chartConfig = {
  views: {
    label: "Traffic",
  },
  traffic: {
    label: "Traffic",
    color: "hsl(var(--chart-2))",
  },
  mobile: {
    label: "Mobile",
    color: "hsl(var(--chart-2))",
  },
};

export function Lineargraph() {
  const [activeChart, setActiveChart] = React.useState("traffic");
  // const [chartData, setChartData] = React.useState([]);

  // const getBarGraphData = async () => {
  //   try{
  //     const res = await invoke("get_ip_stat");
  //     setChartData(res)
  //   }
  //   catch(err){
  //     setChartData([]);
  //     console.log(err);
  //   }
  // }

  const total = React.useMemo(
    () => ({
      traffic: chartData.reduce((acc, curr) => acc + curr.traffic, 0),
      mobile: chartData.reduce((acc, curr) => acc + curr.mobile, 0),
    }),
    []
  );

  return (
    <Card>
      <CardHeader className="flex flex-col items-stretch space-y-0 border-b p-0 sm:flex-row">
        <div className="flex flex-1 flex-col justify-center gap-1 px-6 py-5 sm:py-6">
          <CardTitle>Line Chart</CardTitle>
          <CardDescription>
            Showing Network Traffic within particular timestamps
          </CardDescription>
        </div>
        <div className="flex">
          {/* {["traffic", "mobile"].map((key) => { */}
          {["traffic"].map((key) => {
            const chart = key;
            return (
              <button
                key={chart}
                data-active={activeChart === chart}
                className="flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left even:border-l data-[active=true]:bg-muted/50 sm:border-l sm:border-t-0 sm:px-8 sm:py-6"
                onClick={() => setActiveChart(chart)}
              >
                <span className="text-xs text-muted-foreground">
                  {chartConfig[chart].label}
                </span>
                <span className="text-lg font-bold leading-none sm:text-3xl">
                  {total[key].toLocaleString()}
                </span>
              </button>
            );
          })}
        </div>
      </CardHeader>
      <CardContent className="px-2 sm:p-6">
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <LineChart
            accessibilityLayer
            data={chartData}
            margin={{
              left: 12,
              right: 12,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="timeStamp"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              minTickGap={10}
              tickFormatter={(value) => {
                return value;
              }}
              label={{
                value: "TimeStamp",
                position: "centre",
                dy: 7,
              }}
            />
            <YAxis
              dataKey="traffic"
              tickLine={false}
              axisLine={false}
              tickMargin={15}
              tickCount={5}
              tickFormatter={(value) => {
                return value;
              }}
              label={{
                value: "Traffic",
                position: "centre",
                dy: -15,
              }}
            />
            <ChartTooltip
              content={
                <ChartTooltipContent
                  className="w-[150px]"
                  nameKey="views"
                  labelFormatter={(value) => {
                    return value;
                  }}
                />
              }
            />
            <Line
              dataKey={activeChart}
              type="monotone"
              stroke={`var(--color-${activeChart})`}
              strokeWidth={2}
              dot={false}
            />
          </LineChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
