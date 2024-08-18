import * as React from "react";
import { Label, Pie, PieChart, Sector,Cell } from "recharts";
// import { PieSectorDataItem } from "recharts/types/polar/Pie";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  ChartContainer,
  ChartStyle,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const chartConfig = {
  Protocol: {
    label: "Protocol",
  },
  desktop: {
    label: "Desktop",
  },
  IPV4: {
    label: "IPv4",
    color: "#914F1E",
  },
  IPV6: {
    label: "IPv6",
    color: "#FFC107",
  },
};
const COLORS = {
  IPv4: "hsl(var(--chart-2))",
  IPv6: "hsl(var(--chart-5))"
};

export function Piechart({ data }) {
  const [activeType, setActiveType] = React.useState(data && data.length > 0 ? data[0].type : '');

  const activeIndex = React.useMemo(() => {
    if (!data || data.length === 0) return 0;
    const index = data.findIndex((item) => item.type === activeType);
    return index >= 0 ? index : 0;
  }, [activeType, data]);

  const types = React.useMemo(() => data ? data.map((item) => item.type) : [], [data]);

  const chartConfig = React.useMemo(() => {
    if (!data) return {};
    return data.reduce((acc, item) => {
      acc[item.type] = {
        label: item.type,
      };
      return acc;
    }, {});
  }, [data]);

  console.table(data)
  if (!data || data.length === 0) {
    return <div>No data available</div>;
  }

  return (
    <Card data-chart="pie-interactive" className="flex flex-col">
      <ChartStyle id="pie-interactive" config={chartConfig} />
      <CardHeader className="flex-row items-start space-y-0 pb-0">
        <div className="grid gap-1">
          <CardTitle>Pie Chart</CardTitle>
          <CardDescription>Packet Types Distribution</CardDescription>
        </div>
        <Select value={activeType} onValueChange={setActiveType}>
          <SelectTrigger
            className="ml-auto h-7 w-[130px] rounded-lg pl-2.5"
            aria-label="Select a value"
          >
            <SelectValue placeholder="Select type" />
          </SelectTrigger>
          <SelectContent align="end" className="rounded-xl">
            {types.map((key) => {
              const config = chartConfig[key];

              if (!config) {
                return null;
              }
                console.log(key);
                return (
                  <SelectItem
                    key={key}
                    value={key}
                    className="rounded-lg [&_span]:flex"
                  >
                    <div className="flex items-center gap-2 text-xs">
                      <span
                        className="flex h-3 w-3 shrink-0 rounded-sm"
                        style={{
                          backgroundColor: key === "IPv4" ? "hsl(var(--chart-2))" : key === "IPv6" ? "hsl(var(--chart-5))" : undefined
                        }}
                      />
                      {config?.label}
                    </div>
                  </SelectItem>
                );
            })}
          </SelectContent>
        </Select>
      </CardHeader>
      <CardContent className="flex flex-1 justify-center pb-0">
        <ChartContainer
          id="pie-interactive"
          config={chartConfig}
          className="mx-auto aspect-square w-full max-w-[300px]"
        >
          <PieChart>
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent hideLabel />}
            />

            {console.log(data[0].type)}
            {/* {console.log(dataKey)}  */}
            <Pie
              data={data}
              dataKey="count"
              nameKey="type"
              innerRadius={60}
              strokeWidth={5}
              //style={ {backgroundColor:"IPv4" ? "#914F1E" :  "IPv6" ? "#FFC107" : undefined}}

              // fill={"IPv4" ? "#914F1E" : "#FFC107"}
              // fill= {"IPv6" ? "#FFC107" : "#FFC107"}
              activeIndex={activeIndex}
              activeShape={({ outerRadius = 0, ...props }) => (
                <g>
                  <Sector {...props} outerRadius={outerRadius + 10} />
                  <Sector
                    {...props}
                    outerRadius={outerRadius + 25}
                    innerRadius={outerRadius + 12}
                  />
                </g>
              )}
            >
              {data.map((entry, index) => (
                <Cell key={`cell-${index}`} fill={COLORS[entry.type] || "#8884d8"} />
              ))}

                   
              <Label
                content={({ viewBox }) => {
                  if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                    const count = (data[activeIndex] && data[activeIndex].count) || 0;
                    return (
                      <text
                        x={viewBox.cx}
                        y={viewBox.cy}
                        textAnchor="middle"
                        dominantBaseline="middle"
                      >
                        <tspan
                          x={viewBox.cx}
                          y={viewBox.cy}
                          className="fill-foreground text-3xl font-bold"
                        >
                          {count.toLocaleString()}
                        </tspan>
                        <tspan
                          x={viewBox.cx}
                          y={(viewBox.cy || 0) + 24}
                          className="fill-muted-foreground"
                        >
                          Packets
                        </tspan>
                      </text>
                    );
                  }
                  return null;
                }}
              />
            </Pie>
          </PieChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
