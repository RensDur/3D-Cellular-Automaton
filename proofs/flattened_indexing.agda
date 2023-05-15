open import Agda.Builtin.Nat public
open import Agda.Builtin.Equality public

-- symmetry of equality
sym : {A : Set} {x y : A} → x ≡ y → y ≡ x
sym refl = refl

-- transitivity of equality
trans : {A : Set} {x y z : A} → x ≡ y → y ≡ z → x ≡ z
trans refl refl = refl

-- congruence of equality
cong : {A B : Set} {x y : A} → (f : A → B) → x ≡ y → f x ≡ f y
cong f refl = refl

begin_ : {A : Set} → {x y : A} → x ≡ y → x ≡ y
begin p = p

_end : {A : Set} → (x : A) → x ≡ x
x end = refl

_=⟨_⟩_ : {A : Set} → (x : A) → {y z : A}
       → x ≡ y → y ≡ z → x ≡ z
x =⟨ p ⟩ q = trans p q

_=⟨⟩_ : {A : Set} → (x : A) → {y : A} → x ≡ y → x ≡ y
x =⟨⟩ q = x =⟨ refl ⟩ q

infix   1  begin_
infix   3  _end
infixr  2  _=⟨_⟩_
infixr  2  _=⟨⟩_

data List (A : Set) : Set where
  []    : List A
  _::_  : A → List A → List A

length : {A : Set} → List A → Nat
length []         = 0
length (x :: xs)  = suc (length xs)

_++_ : {A : Set} → List A → List A → List A
[]         ++  ys  = ys
(x :: xs)  ++  ys  = x :: (xs ++ ys)

map : {A B : Set} → (A → B) → List A → List B
map f []         = []
map f (x :: xs)  = f x :: map f xs






-- This proves that delta-gid = delta-x + delta-y*size + delta-z*size^2

length-map : {A B : Set} (f : A → B) (xs : List A) → length (map f xs) ≡ length xs
length-map {A} {B} f [] =
    begin
        length (map f [])
    =⟨⟩
        length []
    end

length-map {A} {B} f (x :: xs) =
    begin
        length (map f (x :: xs))
    =⟨⟩
        length (f x :: map f xs)
    =⟨⟩
        suc (length (map f xs))
    =⟨ cong suc (length-map f xs) ⟩
        suc (length xs)
    =⟨⟩
        length (x :: xs)
    end


proof : (x : Nat) (y : Nat) (z : Nat) (dx : Nat) (dy : Nat) (dz : Nat) (size : Nat) → (x + dx) + size * (y + dy) + size * size * (z + dz) ≡ (x + y * size + z * size * size) + (dx + dy * size + dz * size * size)
proof (suc x) (suc y) (suc z) (suc dx) (suc dy) (suc dz) zero =
    begin
        ((suc x) + (suc dx)) + ((suc y) + (suc dy)) * zero + ((suc z) + (suc dz)) * zero * zero
    =⟨⟩
        