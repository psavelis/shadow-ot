'use client'

import { motion } from 'framer-motion'
import { MessageCircle, Users, ExternalLink } from 'lucide-react'

export function CommunitySection() {
  return (
    <section className="py-20 lg:py-32 bg-gradient-to-b from-shadow-950 to-shadow-900">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5 }}
          className="max-w-4xl mx-auto text-center"
        >
          <div className="inline-flex items-center space-x-2 bg-shadow-800 rounded-full px-4 py-2 mb-6">
            <Users className="w-4 h-4 text-accent-500" />
            <span className="text-shadow-300 text-sm">15,000+ community members</span>
          </div>

          <h2 className="font-display text-4xl lg:text-5xl font-bold mb-6">
            Join Our <span className="gradient-text">Community</span>
          </h2>

          <p className="text-shadow-300 text-lg mb-10 max-w-2xl mx-auto">
            Connect with thousands of players, share strategies, participate in events,
            and be part of the Shadow OT family.
          </p>

          <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
            <a
              href="https://discord.gg/shadowot"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center space-x-3 px-8 py-4 bg-[#5865F2] hover:bg-[#4752C4] rounded-xl transition-all duration-300 group"
            >
              <MessageCircle className="w-6 h-6" />
              <span className="font-semibold text-lg">Join Discord</span>
              <ExternalLink className="w-4 h-4 opacity-50 group-hover:opacity-100 transition-opacity" />
            </a>

            <a
              href="/forum"
              className="flex items-center space-x-3 px-8 py-4 bg-shadow-800 hover:bg-shadow-700 border border-shadow-600 rounded-xl transition-all duration-300"
            >
              <Users className="w-6 h-6" />
              <span className="font-semibold text-lg">Visit Forums</span>
            </a>
          </div>
        </motion.div>
      </div>
    </section>
  )
}
