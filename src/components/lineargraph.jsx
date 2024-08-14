import * as React from "react";
import { CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts";

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
  { timeStamp: "11:20:19", traffic: 222, mobile: 150 },
  { timeStamp: "11:20:20", traffic: 97, mobile: 180 },
  { timeStamp: "11:20:21", traffic: 167, mobile: 120 },
  { timeStamp: "11:20:22", traffic: 245, mobile: 180 },
  { timeStamp: "11:20:22", traffic: 409, mobile: 320 },
  { timeStamp: "11:20:22", traffic: 59, mobile: 110 },
  { timeStamp: "11:20:22", traffic: 261, mobile: 190 },
  { timeStamp: "11:20:23", traffic: 301, mobile: 340 },
  { timeStamp: "11:20:24", traffic: 242, mobile: 260 },
  { timeStamp: "11:20:25", traffic: 373, mobile: 290 },
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
