'use client'

import { motion } from 'framer-motion'
import { 
  Users, 
  Server, 
  Globe, 
  Activity,
  TrendingUp,
  TrendingDown,
  AlertTriangle,
  CheckCircle,
  Clock,
  Cpu,
  HardDrive,
  MemoryStick,
  ArrowUpRight,
  Play,
  Pause,
  RotateCcw
} from 'lucide-react'
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, LineChart, Line } from 'recharts'

const stats = [
  { name: 'Players Online', value: '5,247', change: '+12%', changeType: 'positive', icon: Users },
  { name: 'Active Realms', value: '6', change: '0', changeType: 'neutral', icon: Globe },
  { name: 'Server Uptime', value: '99.9%', change: '+0.1%', changeType: 'positive', icon: Activity },
  { name: 'Active Sessions', value: '8,432', change: '+8%', changeType: 'positive', icon: Server },
]

const realms = [
  { name: 'Shadowveil', status: 'online', players: 1847, cpu: 45, memory: 62, uptime: '14d 6h' },
  { name: 'Aetheria', status: 'online', players: 2103, cpu: 52, memory: 68, uptime: '14d 6h' },
  { name: 'Warbound', status: 'online', players: 892, cpu: 38, memory: 45, uptime: '14d 6h' },
  { name: 'Mythara', status: 'online', players: 654, cpu: 28, memory: 35, uptime: '14d 6h' },
  { name: 'Voidborne', status: 'online', players: 421, cpu: 22, memory: 28, uptime: '7d 12h' },
  { name: 'Grimhollow', status: 'offline', players: 0, cpu: 0, memory: 0, uptime: '-' },
]

const recentAlerts = [
  { type: 'warning', message: 'High memory usage on Aetheria (68%)', time: '5 min ago' },
  { type: 'info', message: 'Scheduled maintenance in 2 hours', time: '30 min ago' },
  { type: 'success', message: 'Database backup completed', time: '1 hour ago' },
  { type: 'warning', message: 'Unusual login pattern detected', time: '2 hours ago' },
]

const playersData = [
  { time: '00:00', players: 3200 },
  { time: '04:00', players: 1800 },
  { time: '08:00', players: 2400 },
  { time: '12:00', players: 4200 },
  { time: '16:00', players: 5100 },
  { time: '20:00', players: 5800 },
  { time: '24:00', players: 5247 },
]

const performanceData = [
  { time: '00:00', cpu: 32, memory: 45 },
  { time: '04:00', cpu: 28, memory: 42 },
  { time: '08:00', cpu: 35, memory: 48 },
  { time: '12:00', cpu: 48, memory: 58 },
  { time: '16:00', cpu: 52, memory: 62 },
  { time: '20:00', cpu: 55, memory: 65 },
  { time: '24:00', cpu: 45, memory: 55 },
]

export default function AdminDashboard() {
  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <h1 className="text-2xl font-display font-bold mb-1">Admin Dashboard</h1>
        <p className="text-shadow-400">Monitor and manage Shadow OT servers</p>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {stats.map((stat) => (
          <div key={stat.name} className="stat-card">
            <div className="flex items-center justify-between mb-3">
              <div className="w-10 h-10 bg-accent-500/10 rounded-lg flex items-center justify-center">
                <stat.icon className="w-5 h-5 text-accent-500" />
              </div>
              <span className={`flex items-center text-sm font-medium ${
                stat.changeType === 'positive' ? 'text-green-500' : stat.changeType === 'negative' ? 'text-red-500' : 'text-shadow-400'
              }`}>
                {stat.changeType === 'positive' && <TrendingUp className="w-4 h-4 mr-1" />}
                {stat.changeType === 'negative' && <TrendingDown className="w-4 h-4 mr-1" />}
                {stat.change}
              </span>
            </div>
            <h3 className="text-2xl font-bold text-white">{stat.value}</h3>
            <p className="text-shadow-400 text-sm">{stat.name}</p>
          </div>
        ))}
      </motion.div>

      <div className="grid lg:grid-cols-3 gap-6">
        {/* Players Chart */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="lg:col-span-2 card">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-lg font-semibold text-white">Players Online (24h)</h2>
            <span className="text-sm text-shadow-400">Peak: 5,800 at 20:00</span>
          </div>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={playersData}>
                <defs>
                  <linearGradient id="playersGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#e94560" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#e94560" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#3b3c40" />
                <XAxis dataKey="time" stroke="#696a74" fontSize={12} />
                <YAxis stroke="#696a74" fontSize={12} />
                <Tooltip contentStyle={{ backgroundColor: '#1a1a2e', border: '1px solid #3b3c40', borderRadius: '8px' }} />
                <Area type="monotone" dataKey="players" stroke="#e94560" strokeWidth={2} fill="url(#playersGradient)" />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </motion.div>

        {/* Alerts */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.3 }} className="card">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-white">Recent Alerts</h2>
            <AlertTriangle className="w-5 h-5 text-yellow-500" />
          </div>
          <div className="space-y-3">
            {recentAlerts.map((alert, index) => (
              <div key={index} className="flex items-start space-x-3 p-3 bg-shadow-800/50 rounded-lg">
                <div className={`w-2 h-2 rounded-full mt-2 ${
                  alert.type === 'warning' ? 'bg-yellow-500' :
                  alert.type === 'success' ? 'bg-green-500' :
                  alert.type === 'error' ? 'bg-red-500' : 'bg-blue-500'
                }`} />
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-white">{alert.message}</p>
                  <p className="text-xs text-shadow-500">{alert.time}</p>
                </div>
              </div>
            ))}
          </div>
        </motion.div>
      </div>

      {/* Realm Status */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.4 }} className="card">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-lg font-semibold text-white">Realm Status</h2>
          <div className="flex space-x-2">
            <button className="btn-secondary text-sm py-1.5 px-3 flex items-center space-x-1">
              <RotateCcw className="w-4 h-4" /><span>Refresh</span>
            </button>
          </div>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-shadow-700">
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Realm</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Status</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Players</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">CPU</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Memory</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Uptime</th>
                <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Actions</th>
              </tr>
            </thead>
            <tbody>
              {realms.map((realm) => (
                <tr key={realm.name} className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50">
                  <td className="py-3 px-4">
                    <span className="font-medium text-white">{realm.name}</span>
                  </td>
                  <td className="py-3 px-4">
                    <span className={`inline-flex items-center space-x-1.5 px-2 py-1 rounded-full text-xs font-medium ${
                      realm.status === 'online' ? 'bg-green-500/20 text-green-400' : 'bg-red-500/20 text-red-400'
                    }`}>
                      <span className={`w-1.5 h-1.5 rounded-full ${realm.status === 'online' ? 'bg-green-400' : 'bg-red-400'}`} />
                      <span className="capitalize">{realm.status}</span>
                    </span>
                  </td>
                  <td className="py-3 px-4 text-white">{realm.players.toLocaleString()}</td>
                  <td className="py-3 px-4">
                    <div className="flex items-center space-x-2">
                      <div className="w-16 h-2 bg-shadow-700 rounded-full overflow-hidden">
                        <div className={`h-full rounded-full ${realm.cpu > 70 ? 'bg-red-500' : realm.cpu > 50 ? 'bg-yellow-500' : 'bg-green-500'}`} style={{ width: `${realm.cpu}%` }} />
                      </div>
                      <span className="text-sm text-shadow-400">{realm.cpu}%</span>
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex items-center space-x-2">
                      <div className="w-16 h-2 bg-shadow-700 rounded-full overflow-hidden">
                        <div className={`h-full rounded-full ${realm.memory > 70 ? 'bg-red-500' : realm.memory > 50 ? 'bg-yellow-500' : 'bg-green-500'}`} style={{ width: `${realm.memory}%` }} />
                      </div>
                      <span className="text-sm text-shadow-400">{realm.memory}%</span>
                    </div>
                  </td>
                  <td className="py-3 px-4 text-shadow-400">{realm.uptime}</td>
                  <td className="py-3 px-4 text-right">
                    <div className="flex items-center justify-end space-x-2">
                      {realm.status === 'online' ? (
                        <button className="p-1.5 text-yellow-400 hover:bg-yellow-500/20 rounded transition-colors" title="Stop">
                          <Pause className="w-4 h-4" />
                        </button>
                      ) : (
                        <button className="p-1.5 text-green-400 hover:bg-green-500/20 rounded transition-colors" title="Start">
                          <Play className="w-4 h-4" />
                        </button>
                      )}
                      <button className="p-1.5 text-blue-400 hover:bg-blue-500/20 rounded transition-colors" title="Restart">
                        <RotateCcw className="w-4 h-4" />
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </motion.div>

      {/* Performance Chart */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.5 }} className="card">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-lg font-semibold text-white">System Performance (24h)</h2>
          <div className="flex items-center space-x-4 text-sm">
            <span className="flex items-center space-x-2"><span className="w-3 h-3 bg-accent-500 rounded" /><span className="text-shadow-400">CPU</span></span>
            <span className="flex items-center space-x-2"><span className="w-3 h-3 bg-blue-500 rounded" /><span className="text-shadow-400">Memory</span></span>
          </div>
        </div>
        <div className="h-64">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={performanceData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#3b3c40" />
              <XAxis dataKey="time" stroke="#696a74" fontSize={12} />
              <YAxis stroke="#696a74" fontSize={12} domain={[0, 100]} />
              <Tooltip contentStyle={{ backgroundColor: '#1a1a2e', border: '1px solid #3b3c40', borderRadius: '8px' }} />
              <Line type="monotone" dataKey="cpu" stroke="#e94560" strokeWidth={2} dot={false} />
              <Line type="monotone" dataKey="memory" stroke="#4A90D9" strokeWidth={2} dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </motion.div>
    </div>
  )
}

