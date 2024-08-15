import React from "react";
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

const chartConfig = {
  traffic: {
    label: "Traffic",
    color: "hsl(var(--chart-2))",
  },
};

export function Lineargraph({ data }) {
  if (!data || data.length === 0) {
    return <div>No data available</div>;
  }

  const total = {
    traffic: data.reduce((acc, curr) => acc + curr.traffic, 0),
  };

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
          <div
            className="flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left even:border-l data-[active=true]:bg-muted/50 sm:border-l sm:border-t-0 sm:px-8 sm:py-6"
          >
            <span className="text-xs text-muted-foreground">
              {chartConfig.traffic.label}
            </span>
            <span className="text-lg font-bold leading-none sm:text-3xl">
              {total.traffic.toLocaleString()}
            </span>
          </div>
        </div>
      </CardHeader>
      <CardContent className="px-2 sm:p-6">
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <LineChart
            accessibilityLayer
            data={data}
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
                  nameKey="traffic"
                  labelFormatter={(value) => {
                    return value;
                  }}
                />
              }
            />
            <Line
              dataKey="traffic"
              type="monotone"
              stroke={`var(--color-traffic)`}
              strokeWidth={2}
              dot={false}
            />
          </LineChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}