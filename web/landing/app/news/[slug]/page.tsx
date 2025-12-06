'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { Calendar, User, Tag, Heart, MessageSquare, Share2, ArrowLeft, Clock } from 'lucide-react'

// This would normally fetch from API based on slug
const article = {
  id: '1',
  slug: 'winter-event-2024',
  title: 'Winter Event 2024 - Frozen Realms Await!',
  excerpt: 'The most anticipated event of the year is here! Explore frozen dungeons, defeat ice bosses, and earn exclusive winter-themed rewards.',
  content: `
## The Frozen Realm Awaits

Winter has arrived in Shadow OT, and with it comes our most ambitious seasonal event yet! From **December 1st through January 15th**, all realms will be transformed into winter wonderlands with exclusive content, challenges, and rewards.

### New Frozen Dungeons

We've added three new dungeons specifically for the Winter Event:

1. **Ice Caverns of Norheim** - A sprawling underground complex filled with frost elementals and ancient ice constructs. Recommended for levels 150+.

2. **The Frozen Citadel** - An ancient fortress encased in eternal ice, home to the fearsome Frost King. Party of 5+ recommended, levels 200+.

3. **Blizzard Peak** - Scale the highest mountain in Aetheria and face the legendary Storm Dragon at its peak. Raid content for 10+ players, level 250+.

### Exclusive Winter Rewards

Complete event challenges to earn these limited-time items:

| Reward | Requirement |
|--------|-------------|
| Frost Walker Mount | Complete all dungeons |
| Snowflake Outfit | Defeat Frost King 10 times |
| Ice Crown | Reach top 100 in event rankings |
| Winter Sprite Pet | Collect 1000 Snowflakes |

### Daily Challenges

Every day during the event, special daily challenges will be available:

- **Snowball Fight** - Participate in PvP snowball battles
- **Ice Fishing** - Catch rare frozen fish
- **Gift Delivery** - Help NPCs deliver presents
- **Snow Sculpture Contest** - Create and vote on snow sculptures

### Event Currency

Collect **Snowflakes** from monsters, quests, and challenges to spend at the Winter Shop. New items rotate weekly, so check back often!

### Technical Notes

- Event dungeons scale with party size
- Special winter effects may impact older hardware - check video settings
- Event items are tradeable between players
- Rankings reset daily at midnight server time

We hope you enjoy the Winter Event 2024! See you in the frozen realms!

*- The Shadow OT Team*
  `,
  category: 'event',
  author: { name: 'Shadow Team', avatar: 'https://placehold.co/100/1a1a2e/e94560?text=ST' },
  publishedAt: '2024-12-01T10:00:00Z',
  updatedAt: '2024-12-02T15:30:00Z',
  featured: true,
  image: 'https://placehold.co/1200x600/1a1a2e/e94560?text=Winter+Event+2024',
  reactions: { likes: 342, comments: 89, shares: 45 },
  tags: ['event', 'seasonal', 'rewards', 'dungeons', 'winter'],
  readTime: 4,
}

const relatedArticles = [
  { id: '2', slug: 'patch-2-5-combat-rebalance', title: 'Patch 2.5 - Major Combat Rebalance', category: 'update' },
  { id: '3', slug: 'new-realm-grimhollow', title: 'Introducing Grimhollow - A New Horror-Themed Realm', category: 'announcement' },
]

const categoryColors: Record<string, string> = {
  announcement: 'bg-yellow-500/20 text-yellow-400',
  update: 'bg-blue-500/20 text-blue-400',
  event: 'bg-purple-500/20 text-purple-400',
  community: 'bg-green-500/20 text-green-400',
  maintenance: 'bg-orange-500/20 text-orange-400',
  development: 'bg-cyan-500/20 text-cyan-400',
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

export default function NewsArticlePage({ params }: { params: { slug: string } }) {
  return (
    <div className="min-h-screen bg-shadow-950">
      {/* Hero Image */}
      <div className="relative h-[40vh] md:h-[50vh]">
        <div className="absolute inset-0 bg-gradient-to-t from-shadow-950 via-shadow-950/60 to-transparent z-10" />
        <img
          src={article.image}
          alt={article.title}
          className="w-full h-full object-cover"
        />
        <div className="absolute bottom-0 left-0 right-0 z-20 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 pb-12">
          <Link href="/news" className="inline-flex items-center gap-2 text-shadow-400 hover:text-white mb-4 transition-colors">
            <ArrowLeft className="w-4 h-4" />
            Back to News
          </Link>
          <span className={`inline-block px-3 py-1 rounded text-sm font-medium mb-4 ${categoryColors[article.category]}`}>
            {article.category.charAt(0).toUpperCase() + article.category.slice(1)}
          </span>
          <h1 className="font-display text-3xl md:text-4xl lg:text-5xl font-bold mb-4">{article.title}</h1>
          <div className="flex flex-wrap items-center gap-4 text-shadow-400 text-sm">
            <span className="flex items-center gap-2">
              <img src={article.author.avatar} alt={article.author.name} className="w-6 h-6 rounded-full" />
              {article.author.name}
            </span>
            <span className="flex items-center gap-1">
              <Calendar className="w-4 h-4" />
              {formatDate(article.publishedAt)}
            </span>
            <span className="flex items-center gap-1">
              <Clock className="w-4 h-4" />
              {article.readTime} min read
            </span>
          </div>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="grid lg:grid-cols-4 gap-8">
          {/* Main Content */}
          <motion.article
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="lg:col-span-3"
          >
            <div className="prose prose-invert prose-lg max-w-none">
              <p className="lead text-xl text-shadow-300 mb-8">{article.excerpt}</p>
              <div 
                className="article-content"
                dangerouslySetInnerHTML={{ __html: article.content.replace(/\n/g, '<br/>') }}
              />
            </div>

            {/* Tags */}
            <div className="flex flex-wrap gap-2 mt-8 pt-8 border-t border-shadow-700">
              {article.tags.map((tag) => (
                <Link
                  key={tag}
                  href={`/news?tag=${tag}`}
                  className="px-3 py-1 bg-shadow-800 text-shadow-400 rounded-full text-sm hover:bg-shadow-700 hover:text-white transition-colors"
                >
                  #{tag}
                </Link>
              ))}
            </div>

            {/* Reactions */}
            <div className="flex items-center gap-6 mt-8 pt-8 border-t border-shadow-700">
              <button className="flex items-center gap-2 text-shadow-400 hover:text-accent-400 transition-colors">
                <Heart className="w-5 h-5" />
                <span>{article.reactions.likes}</span>
              </button>
              <button className="flex items-center gap-2 text-shadow-400 hover:text-accent-400 transition-colors">
                <MessageSquare className="w-5 h-5" />
                <span>{article.reactions.comments}</span>
              </button>
              <button className="flex items-center gap-2 text-shadow-400 hover:text-accent-400 transition-colors">
                <Share2 className="w-5 h-5" />
                <span>Share</span>
              </button>
            </div>

            {/* Comments Section */}
            <div className="mt-12">
              <h3 className="font-display text-xl font-bold mb-6">Comments ({article.reactions.comments})</h3>
              <div className="card mb-6">
                <textarea
                  placeholder="Write a comment..."
                  className="w-full bg-shadow-800 border border-shadow-600 rounded-lg px-4 py-3 text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50 resize-none"
                  rows={3}
                />
                <div className="flex justify-end mt-3">
                  <button className="btn-primary">Post Comment</button>
                </div>
              </div>
              <p className="text-shadow-500 text-center py-8">Login to view and post comments</p>
            </div>
          </motion.article>

          {/* Sidebar */}
          <motion.aside
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2 }}
            className="lg:col-span-1 space-y-6"
          >
            <div className="card sticky top-24">
              <h3 className="font-semibold text-white mb-4">Related Articles</h3>
              <div className="space-y-4">
                {relatedArticles.map((related) => (
                  <Link key={related.id} href={`/news/${related.slug}`} className="block group">
                    <span className={`inline-block px-2 py-0.5 rounded text-xs font-medium mb-1 ${categoryColors[related.category]}`}>
                      {related.category}
                    </span>
                    <h4 className="text-sm text-white group-hover:text-accent-400 transition-colors">
                      {related.title}
                    </h4>
                  </Link>
                ))}
              </div>
            </div>
          </motion.aside>
        </div>
      </div>
    </div>
  )
}

