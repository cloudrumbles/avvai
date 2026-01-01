import type { Card } from '../types'
import { cards } from './cards'

type Props = {
  card: Card
}

export function LessonCard(props: Props) {
  const Component = cards[props.card.type]
  return <Component {...props.card} />
}
