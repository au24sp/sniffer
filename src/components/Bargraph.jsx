import React, { useState, useEffect } from "react";
import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from "recharts";
import { ChartContainer } from "@/components/ui/chart";
import { ChartTooltip, ChartTooltipContent } from "@/components/ui/chart";
import { ChartLegend, ChartLegendContent } from "@/components/ui/chart";

const chartConfig = {
  Source: {
    label: "Source",
    color: "hsl(var(--chart-1))",
  },
  Destination: {
    label: "Destination",
    color: "hsl(var(--chart-5))",
  },
};

export function Bargraph({ data }) {
  if (!data || data.length === 0) {
    return <div>No data available</div>;
  }

  return (
    <ChartContainer config={chartConfig} className="min-h-[200px] w-full">
      <BarChart accessibilityLayer data={data}>
        <CartesianGrid vertical={false} />
        <XAxis
          dataKey="IP"
          tickLine={true}
          tickMargin={20}
          axisLine={false}
          tickFormatter={(value) => value.slice(0, 14)}
          label={{ value: "IP Address", position: "insideBottom", dy: -5 }}
        />
        <YAxis
          tickLine={true}
          tickMargin={10}
          axisLine={false}
          label={{ value: "Traffic", position: "centre", dy: 20 }}
        />
        <ChartTooltip content={<ChartTooltipContent />} />
        <ChartLegend content={<ChartLegendContent />} />
        <Bar dataKey="Source" fill="var(--color-Source)" radius={4} />
        <Bar dataKey="Destination" fill="var(--color-Destination)" radius={4} />
      </BarChart>
    </ChartContainer>
  );
}