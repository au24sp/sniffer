import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from "recharts";
import { ChartContainer } from "@/components/ui/chart";
import { ChartTooltip, ChartTooltipContent } from "@/components/ui/chart";
import { ChartLegend, ChartLegendContent } from "@/components/ui/chart";


const chartData = [
  { IP: "192.168.31.127", Source: 186, Destination: 80 },
  { IP: "192.168.31.128", Source: 305, Destination: 200 },
  { IP: "192.168.31.129", Source: 237, Destination: 120 },
  { IP: "192.168.31.130", Source: 73, Destination: 190 },
  { IP: "192.168.31.131", Source: 209, Destination: 130 },
  { IP: "192.168.31.132", Source: 214, Destination: 140 },
  { IP: "192.168.31.133", Source: 160, Destination: 100 },
  { IP: "192.168.31.134", Source: 240, Destination: 150 },
  { IP: "192.168.31.135", Source: 180, Destination: 90 },
  { IP: "192.168.31.136", Source: 120, Destination: 60 },
  { IP: "192.168.31.137", Source: 140, Destination: 70 },
  { IP: "192.168.31.138", Source: 180, Destination: 90 },
];

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

export function Bargraph() {
  return (
    <ChartContainer config={chartConfig} className="min-h-[200px] w-full">
      <BarChart accessibilityLayer data={chartData}>
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
