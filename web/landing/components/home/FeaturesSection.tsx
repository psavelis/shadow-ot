'use client'

import { motion } from 'framer-motion'
import {
  Globe,
  Zap,
  Shield,
  Users,
  Gamepad2,
  Map,
  Coins,
  Trophy,
  Swords,
  Home,
  MessageSquare,
  Bot,
  Palette,
  BookOpen,
  Target,
  Sparkles
} from 'lucide-react'

const features = [
  {
    icon: Globe,
    title: 'Multi-Realm Architecture',
    description: 'One account, multiple realms. Choose your playstyle across themed servers.',
    color: 'text-blue-400',
    bg: 'bg-blue-500/10',
  },
  {
    icon: Zap,
    title: 'Ultra-Low Latency',
    description: 'Optimized Rust backend with global edge servers for minimal ping.',
    color: 'text-yellow-400',
    bg: 'bg-yellow-500/10',
  },
  {
    icon: Shield,
    title: 'Advanced Anti-Cheat',
    description: 'AI-powered detection system keeping the game fair for everyone.',
    color: 'text-green-400',
    bg: 'bg-green-500/10',
  },
  {
    icon: Users,
    title: 'Cross-Realm Events',
    description: 'Compete with players from all realms in epic tournaments and events.',
    color: 'text-purple-400',
    bg: 'bg-purple-500/10',
  },
  {
    icon: Coins,
    title: 'In-Game Market',
    description: 'Full-featured market system with cross-realm trading capabilities.',
    color: 'text-amber-400',
    bg: 'bg-amber-500/10',
  },
  {
    icon: Trophy,
    title: 'Matchmaking & Ranked',
    description: 'Competitive PvP with ELO-based matchmaking and seasonal rankings.',
    color: 'text-orange-400',
    bg: 'bg-orange-500/10',
  },
  {
    icon: Swords,
    title: 'Guild Wars',
    description: 'Organize guild battles, capture territories, and dominate the realm.',
    color: 'text-red-400',
    bg: 'bg-red-500/10',
  },
  {
    icon: Home,
    title: 'Housing System',
    description: 'Purchase and customize houses with auctions and rental system.',
    color: 'text-cyan-400',
    bg: 'bg-cyan-500/10',
  },
  {
    icon: MessageSquare,
    title: 'Forums & Community',
    description: 'Integrated forums, guilds pages, and social features.',
    color: 'text-pink-400',
    bg: 'bg-pink-500/10',
  },
  {
    icon: Bot,
    title: 'Bot-Friendly Mode',
    description: 'Authorized bot support for designated training areas and grinding.',
    color: 'text-indigo-400',
    bg: 'bg-indigo-500/10',
  },
  {
    icon: Map,
    title: 'Custom Map Maker',
    description: 'Create and submit custom maps, monsters, and quests to the community.',
    color: 'text-emerald-400',
    bg: 'bg-emerald-500/10',
  },
  {
    icon: Palette,
    title: 'Custom Assets',
    description: 'Realm-specific sprites, outfits, mounts, and visual effects.',
    color: 'text-rose-400',
    bg: 'bg-rose-500/10',
  },
  {
    icon: BookOpen,
    title: 'Complete Bestiary',
    description: 'Track all creatures with charm points and unlock special abilities.',
    color: 'text-violet-400',
    bg: 'bg-violet-500/10',
  },
  {
    icon: Target,
    title: 'Prey System',
    description: 'Hunt specific creatures for bonus experience and loot.',
    color: 'text-lime-400',
    bg: 'bg-lime-500/10',
  },
  {
    icon: Sparkles,
    title: 'Achievements',
    description: 'Hundreds of achievements with exclusive rewards and titles.',
    color: 'text-fuchsia-400',
    bg: 'bg-fuchsia-500/10',
  },
  {
    icon: Gamepad2,
    title: 'Multi-Client Support',
    description: 'Compatible with official clients 8.6 to 12.x and custom clients.',
    color: 'text-teal-400',
    bg: 'bg-teal-500/10',
  },
]

export function FeaturesSection() {
  return (
    <section className="py-20 lg:py-32 bg-gradient-to-b from-shadow-950 to-shadow-900">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5 }}
          className="text-center mb-16"
        >
          <h2 className="section-title">
            Everything You <span className="gradient-text">Need</span>
          </h2>
          <p className="section-subtitle">
            The most feature-complete Open Tibia server with everything from the real Tibia and much more.
          </p>
        </motion.div>

        {/* Features Grid */}
        <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
          {features.map((feature, index) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: index * 0.05 }}
              className="card-hover group"
            >
              <div className={`w-12 h-12 ${feature.bg} rounded-xl flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300`}>
                <feature.icon className={`w-6 h-6 ${feature.color}`} />
              </div>
              <h3 className="font-semibold text-white text-lg mb-2">{feature.title}</h3>
              <p className="text-shadow-400 text-sm">{feature.description}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  )
}
